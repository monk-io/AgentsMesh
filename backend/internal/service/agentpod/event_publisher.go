package agentpod

import (
	"context"
	"log/slog"

	"github.com/anthropics/agentsmesh/backend/internal/infra/eventbus"
)

// PodEventType defines pod event types for the publisher interface
type PodEventType string

const (
	PodEventCreated       PodEventType = "created"
	PodEventStatusChanged PodEventType = "status_changed"
	PodEventAgentChanged  PodEventType = "agent_changed"
	PodEventTerminated    PodEventType = "terminated"
)

// EventPublisher defines the interface for publishing pod events
// This allows the service to be decoupled from the eventbus implementation
type EventPublisher interface {
	PublishPodEvent(ctx context.Context, eventType PodEventType, orgID int64, podKey, status, previousStatus, agentStatus string)
	PublishPodErrorEvent(ctx context.Context, orgID int64, podKey, previousStatus, errorCode, errorMessage string)
}

// EventBusPublisher implements EventPublisher using EventBus
type EventBusPublisher struct {
	eventBus *eventbus.EventBus
	logger   *slog.Logger
}

// NewEventBusPublisher creates a new EventBusPublisher
func NewEventBusPublisher(eventBus *eventbus.EventBus, logger *slog.Logger) *EventBusPublisher {
	if logger == nil {
		logger = slog.Default()
	}
	return &EventBusPublisher{
		eventBus: eventBus,
		logger:   logger.With("component", "pod_event_publisher"),
	}
}

// PublishPodEvent publishes a pod event
func (p *EventBusPublisher) PublishPodEvent(ctx context.Context, eventType PodEventType, orgID int64, podKey, status, previousStatus, agentStatus string) {
	if p.eventBus == nil {
		return
	}

	// Map PodEventType to eventbus.EventType (compile-time type safety)
	var et eventbus.EventType
	switch eventType {
	case PodEventCreated:
		et = eventbus.EventPodCreated
	case PodEventStatusChanged:
		et = eventbus.EventPodStatusChanged
	case PodEventAgentChanged:
		et = eventbus.EventPodAgentChanged
	case PodEventTerminated:
		et = eventbus.EventPodTerminated
	default:
		p.logger.Warn("unknown pod event type", "type", eventType)
		return
	}

	data := &eventbus.PodStatusChangedData{
		PodKey:         podKey,
		Status:         status,
		PreviousStatus: previousStatus,
		AgentStatus:    agentStatus,
	}

	event, err := eventbus.NewEntityEvent(et, orgID, "pod", podKey, data)
	if err != nil {
		p.logger.Error("failed to create pod event",
			"error", err,
			"type", eventType,
			"pod_key", podKey,
		)
		return
	}

	if err := p.eventBus.Publish(ctx, event); err != nil {
		p.logger.Error("failed to publish pod event",
			"error", err,
			"type", eventType,
			"pod_key", podKey,
		)
	}
}

// PublishPodErrorEvent publishes a pod error event with error code and message.
func (p *EventBusPublisher) PublishPodErrorEvent(ctx context.Context, orgID int64, podKey, previousStatus, errorCode, errorMessage string) {
	if p.eventBus == nil {
		return
	}

	data := &eventbus.PodStatusChangedData{
		PodKey:         podKey,
		Status:         "error",
		PreviousStatus: previousStatus,
		ErrorCode:      errorCode,
		ErrorMessage:   errorMessage,
	}

	event, err := eventbus.NewEntityEvent(eventbus.EventPodStatusChanged, orgID, "pod", podKey, data)
	if err != nil {
		p.logger.Error("failed to create pod error event", "error", err, "pod_key", podKey)
		return
	}

	if err := p.eventBus.Publish(ctx, event); err != nil {
		p.logger.Error("failed to publish pod error event", "error", err, "pod_key", podKey)
	}
}
