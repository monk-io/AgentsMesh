package v1

import (
	agentSvc "github.com/anthropics/agentsmesh/backend/internal/service/agent"
)

// MessageHandler backs the agent-message DLQ admin endpoints only. The
// agent-message send/get/mark-read REST surface was dropped — proto.channel.v1
// owns the channel messaging wire and proto-driven pod messaging has its own
// service. The DLQ stays REST-only because it's an ops/debug surface with no
// client-side mirror.
type MessageHandler struct {
	msgSvc *agentSvc.MessageService
}

func NewMessageHandler(msgSvc *agentSvc.MessageService) *MessageHandler {
	return &MessageHandler{
		msgSvc: msgSvc,
	}
}
