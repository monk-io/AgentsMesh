package v1

import (
	"log/slog"

	"github.com/gin-gonic/gin"
)

func registerBillingRoutes(rg *gin.RouterGroup, svc *Services) {
	RegisterBillingHandlers(rg.Group("/billing"), svc.Billing)
}

func registerInvitationRoutes(rg *gin.RouterGroup, svc *Services) {
	if svc.Invitation != nil {
		invitationHandler := NewInvitationHandler(svc.Invitation, svc.Org, svc.User, svc.Billing)
		invitationHandler.RegisterOrgRoutes(rg)
	}
}

func registerFileRoutes(rg *gin.RouterGroup, svc *Services) {
	if svc.File == nil {
		slog.Warn("File service is nil, file routes not registered")
		return
	}
	slog.Info("Registering file routes", "service", "file")
	fileHandler := NewFileHandler(svc.File)
	files := rg.Group("/files")
	{
		files.POST("/presign", fileHandler.PresignUpload)
	}
}
