package v1

import "github.com/gin-gonic/gin"

func registerTicketRoutes(rg *gin.RouterGroup, svc *Services) {
	meshHandler := NewMeshHandler(svc.Mesh, svc.Ticket)
	tickets := rg.Group("/tickets")
	{
		// proto.ticket.v1 owns the CRUD/board/labels/assignees/sub-tickets
		// surface — see backend/internal/api/connect/ticket. Only ticket→pod
		// lookup stays here (MeshService domain).
		tickets.POST("/batch-pods", meshHandler.BatchGetTicketPods)
		tickets.GET("/:ticket_slug/pods", meshHandler.GetTicketPods)
		tickets.POST("/:ticket_slug/pods", meshHandler.CreatePodForTicket)
	}

	meshGroup := rg.Group("/mesh")
	{
		meshGroup.GET("/topology", meshHandler.GetTopology)
	}
}
