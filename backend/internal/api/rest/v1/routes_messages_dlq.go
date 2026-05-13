package v1

import "github.com/gin-gonic/gin"

// registerMessageRoutes mounts the agent-message DLQ (admin/observability
// surface). All other agent-message endpoints were dropped — proto.channel.v1
// owns the channel messaging wire; this route group only retains the dead
// letter queue inspection / replay endpoints used by ops tooling.
func registerMessageRoutes(rg *gin.RouterGroup, svc *Services) {
	if svc.Message == nil {
		return
	}
	messageHandler := NewMessageHandler(svc.Message)
	messages := rg.Group("/messages")
	{
		messages.GET("/dlq", messageHandler.GetDeadLetters)
		messages.POST("/dlq/:id/replay", messageHandler.ReplayDeadLetter)
	}
}
