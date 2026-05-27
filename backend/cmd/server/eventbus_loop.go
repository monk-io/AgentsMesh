package main

import (
	"context"
	"log/slog"
	"time"

	"google.golang.org/protobuf/encoding/protojson"

	"github.com/anthropics/agentsmesh/backend/internal/domain/agentpod"
	"github.com/anthropics/agentsmesh/backend/internal/infra/eventbus"
	"github.com/anthropics/agentsmesh/backend/internal/service/instance"
	loop "github.com/anthropics/agentsmesh/backend/internal/service/loop"
	eventsv1 "github.com/anthropics/agentsmesh/proto/gen/go/events/v1"
)

func setupLoopEventSubscriptions(eventBus *eventbus.EventBus, loopOrchestrator *loop.LoopOrchestrator) {
	eventBus.Subscribe(eventbus.EventPodTerminated, func(event *eventbus.Event) {
		var data eventsv1.PodStatusChangedEventData
		if err := protojson.Unmarshal(event.Data, &data); err != nil {
			slog.Error("failed to unmarshal pod terminated event for loop", "error", err)
			return
		}

		now := time.Now()
		loopOrchestrator.HandlePodTerminated(context.Background(), data.PodKey, data.Status, &now)
	})

	eventBus.Subscribe(eventbus.EventPodStatusChanged, func(event *eventbus.Event) {
		var data eventsv1.PodStatusChangedEventData
		if err := protojson.Unmarshal(event.Data, &data); err != nil {
			return
		}

		switch data.Status {
		case agentpod.StatusCompleted, agentpod.StatusError:
			now := time.Now()
			loopOrchestrator.HandlePodTerminated(context.Background(), data.PodKey, data.Status, &now)
		}
	})

	// Autopilot status changed → detect terminal phases and handle completion.
	// Single path for Autopilot termination detection:
	//   Runner gRPC → PodCoordinator → onAutopilotStatusChange callback → EventAutopilotStatusChanged
	eventBus.Subscribe(eventbus.EventAutopilotStatusChanged, func(event *eventbus.Event) {
		var data eventsv1.AutopilotStatusChangedEventData
		if err := protojson.Unmarshal(event.Data, &data); err != nil {
			return
		}

		switch data.Phase {
		case agentpod.AutopilotPhaseCompleted, agentpod.AutopilotPhaseFailed, agentpod.AutopilotPhaseStopped:
			loopOrchestrator.HandleAutopilotTerminated(context.Background(), data.AutopilotControllerKey, data.Phase)
		}
	})

	slog.Info("Loop event subscriptions registered")
}

func setupOrgAwarenessRefresh(eventBus *eventbus.EventBus, orgAwareness *instance.OrgAwarenessService) {
	eventBus.Subscribe(eventbus.EventRunnerOnline, func(event *eventbus.Event) {
		orgAwareness.Refresh()
	})

	eventBus.Subscribe(eventbus.EventRunnerOffline, func(event *eventbus.Event) {
		orgAwareness.Refresh()
	})

	slog.Info("OrgAwareness runner event subscriptions registered")
}
