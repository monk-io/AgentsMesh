package v1

import (
	"log/slog"

	"github.com/gin-gonic/gin"
)

func registerExtensionRoutes(rg *gin.RouterGroup, svc *Services) {
	if svc.Extension == nil {
		slog.Warn("Extension services not configured, extension routes not registered")
		return
	}

	handler := NewExtensionHandler(svc.Extension)

	repoSkills := rg.Group("/repositories/:id/skills")
	{
		// Multipart upload stays REST (Connect-RPC doesn't handle multipart/form-data).
		repoSkills.POST("/install-from-upload", handler.InstallSkillFromUpload)
	}

	slog.Info("Extension routes registered")
}
