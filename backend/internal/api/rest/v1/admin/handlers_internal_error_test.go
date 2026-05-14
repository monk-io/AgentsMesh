package admin

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/anthropics/agentsmesh/backend/internal/domain/runner"
	adminservice "github.com/anthropics/agentsmesh/backend/internal/service/admin"
	"github.com/gin-gonic/gin"
	"github.com/stretchr/testify/assert"
	"gorm.io/gorm"
)

// =============================================================================
// Dashboard Internal Error Tests
// =============================================================================

func TestDashboardHandler_GetStats_Error(t *testing.T) {
	t.Run("should return 500 when service fails", func(t *testing.T) {
		db := newMockHandlerDB()
		db.countErr = gorm.ErrInvalidDB

		svc := adminservice.NewService(db)
		handler := NewDashboardHandler(svc)

		w := httptest.NewRecorder()
		c := createAdminContext(w)
		c.Request = httptest.NewRequest("GET", "/dashboard/stats", nil)

		handler.GetStats(c)

		assert.Equal(t, http.StatusInternalServerError, w.Code)
	})
}

// =============================================================================
// Runner Internal Error Tests
// =============================================================================

func TestRunnerHandler_ListRunners_Error(t *testing.T) {
	t.Run("should return 500 when service fails", func(t *testing.T) {
		db := newMockHandlerDB()
		db.countErr = gorm.ErrInvalidDB

		svc := adminservice.NewService(db)
		handler := NewRunnerHandler(svc)

		w := httptest.NewRecorder()
		c := createAdminContext(w)
		c.Request = httptest.NewRequest("GET", "/runners", nil)

		handler.ListRunners(c)

		assert.Equal(t, http.StatusInternalServerError, w.Code)
	})
}

func TestRunnerHandler_DisableRunner_InternalError(t *testing.T) {
	t.Run("should return 500 when disable fails", func(t *testing.T) {
		db := newMockHandlerDB()
		db.runners[1] = &runner.Runner{ID: 1, NodeID: "test-node", IsEnabled: true}
		db.saveErr = gorm.ErrInvalidDB

		svc := adminservice.NewService(db)
		handler := NewRunnerHandler(svc)

		w := httptest.NewRecorder()
		c := createAdminContext(w)
		c.Request = httptest.NewRequest("POST", "/runners/1/disable", nil)
		c.Params = gin.Params{{Key: "id", Value: "1"}}

		handler.DisableRunner(c)

		assert.Equal(t, http.StatusInternalServerError, w.Code)
	})
}

func TestRunnerHandler_EnableRunner_InternalError(t *testing.T) {
	t.Run("should return 500 when enable fails", func(t *testing.T) {
		db := newMockHandlerDB()
		db.runners[1] = &runner.Runner{ID: 1, NodeID: "test-node", IsEnabled: false}
		db.saveErr = gorm.ErrInvalidDB

		svc := adminservice.NewService(db)
		handler := NewRunnerHandler(svc)

		w := httptest.NewRecorder()
		c := createAdminContext(w)
		c.Request = httptest.NewRequest("POST", "/runners/1/enable", nil)
		c.Params = gin.Params{{Key: "id", Value: "1"}}

		handler.EnableRunner(c)

		assert.Equal(t, http.StatusInternalServerError, w.Code)
	})
}

func TestRunnerHandler_DeleteRunner_InternalError(t *testing.T) {
	t.Run("should return 500 when delete fails", func(t *testing.T) {
		db := newMockHandlerDB()
		db.runners[1] = &runner.Runner{ID: 1, NodeID: "test-node"}
		db.activePodCount = 0
		db.deleteErr = gorm.ErrInvalidDB

		svc := adminservice.NewService(db)
		handler := NewRunnerHandler(svc)

		w := httptest.NewRecorder()
		c := createAdminContext(w)
		c.Request = httptest.NewRequest("DELETE", "/runners/1", nil)
		c.Params = gin.Params{{Key: "id", Value: "1"}}

		handler.DeleteRunner(c)

		assert.Equal(t, http.StatusInternalServerError, w.Code)
	})
}
