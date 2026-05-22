package v1

import (
	"log/slog"

	"github.com/gin-gonic/gin"
)

// Billing org-scoped REST handlers all migrated to Connect — see
// backend/internal/api/connect/billing. Stub kept so the route group
// composes cleanly with the rest of the org-scoped routes (routes_org_scoped.go).
func registerBillingRoutes(_ *gin.RouterGroup, _ *Services) {}

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
