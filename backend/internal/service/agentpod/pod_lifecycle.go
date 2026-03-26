package agentpod

import (
	"context"
	"errors"
	"fmt"
	"time"

	"github.com/anthropics/agentsmesh/backend/internal/domain/agentpod"
)

// HandlePodCreated handles the pod_created event from runner
func (s *PodService) HandlePodCreated(ctx context.Context, podKey string, ptyPID int, sandboxPath, branchName string) error {
	now := time.Now()
	updates := map[string]interface{}{
		"pty_pid":       ptyPID,
		"status":        agentpod.StatusRunning,
		"started_at":    now,
		"last_activity": now,
	}
	if sandboxPath != "" {
		updates["sandbox_path"] = sandboxPath
	}
	if branchName != "" {
		updates["branch_name"] = branchName
	}
	_, err := s.repo.UpdateByKey(ctx, podKey, updates)
	return err
}

// HandlePodTerminated handles the pod_terminated event from runner
func (s *PodService) HandlePodTerminated(ctx context.Context, podKey string, exitCode *int) error {
	now := time.Now()
	_, err := s.repo.UpdateByKey(ctx, podKey, map[string]interface{}{
		"status":      agentpod.StatusTerminated,
		"finished_at": now,
		"pty_pid":     nil,
	})
	return err
}

// TerminatePod terminates a pod
func (s *PodService) TerminatePod(ctx context.Context, podKey string) error {
	pod, err := s.GetPod(ctx, podKey)
	if err != nil {
		return err
	}

	if !pod.IsActive() {
		return ErrPodTerminated
	}

	previousStatus := pod.Status
	if err := s.UpdatePodStatus(ctx, podKey, agentpod.StatusTerminated); err != nil {
		return err
	}

	if s.eventPublisher != nil {
		s.eventPublisher.PublishPodEvent(
			ctx,
			PodEventTerminated,
			pod.OrganizationID,
			podKey,
			agentpod.StatusTerminated,
			previousStatus,
			"",
		)
	}

	return nil
}

// MarkDisconnected marks a pod as disconnected (user closed browser)
func (s *PodService) MarkDisconnected(ctx context.Context, podKey string) error {
	return s.repo.UpdateByKeyAndStatus(ctx, podKey, agentpod.StatusRunning, map[string]interface{}{
		"status": agentpod.StatusDisconnected,
	})
}

// MarkReconnected marks a pod as running again (user reconnected)
func (s *PodService) MarkReconnected(ctx context.Context, podKey string) error {
	now := time.Now()
	return s.repo.UpdateByKeyAndStatus(ctx, podKey, agentpod.StatusDisconnected, map[string]interface{}{
		"status":        agentpod.StatusRunning,
		"last_activity": now,
	})
}

// RecordActivity records pod activity
func (s *PodService) RecordActivity(ctx context.Context, podKey string) error {
	return s.repo.UpdateField(ctx, podKey, "last_activity", time.Now())
}

// ReconcilePods marks orphaned pods that are not reported by runner
func (s *PodService) ReconcilePods(ctx context.Context, runnerID int64, reportedPodKeys []string) error {
	dbPods, err := s.repo.ListActiveByRunner(ctx, runnerID)
	if err != nil {
		return err
	}

	reportedSet := make(map[string]bool)
	for _, key := range reportedPodKeys {
		reportedSet[key] = true
	}

	now := time.Now()
	var errs []error
	for _, pod := range dbPods {
		if !reportedSet[pod.PodKey] {
			if err := s.repo.MarkOrphaned(ctx, pod, now); err != nil {
				errs = append(errs, fmt.Errorf("mark pod %s orphaned: %w", pod.PodKey, err))
			}
		}
	}

	return errors.Join(errs...)
}

// CleanupStalePods marks stale pods as terminated
func (s *PodService) CleanupStalePods(ctx context.Context, maxIdleHours int) (int64, error) {
	threshold := time.Now().Add(-time.Duration(maxIdleHours) * time.Hour)
	return s.repo.CleanupStale(ctx, threshold)
}

// TimeoutInitializingPods marks pods stuck in "initializing" for longer than
// the given duration as "error" and publishes status change events so the
// frontend receives immediate feedback instead of spinning forever.
func (s *PodService) TimeoutInitializingPods(ctx context.Context, maxInitDuration time.Duration) (int64, error) {
	threshold := time.Now().Add(-maxInitDuration)
	pods, err := s.repo.TimeoutInitializingPods(ctx, threshold)
	if err != nil {
		return 0, err
	}

	// Publish error events so WebSocket clients get notified with error details
	if s.eventPublisher != nil {
		for _, pod := range pods {
			s.eventPublisher.PublishPodErrorEvent(
				ctx,
				pod.OrganizationID,
				pod.PodKey,
				agentpod.StatusInitializing,
				"INIT_TIMEOUT",
				"Pod initialization timed out. The runner may be unreachable or the setup process (git clone, sandbox preparation) may have stalled.",
			)
		}
	}

	return int64(len(pods)), nil
}
