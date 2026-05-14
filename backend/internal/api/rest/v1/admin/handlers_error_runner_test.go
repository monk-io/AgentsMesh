package admin

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/anthropics/agentsmesh/backend/internal/domain/runner"
	adminservice "github.com/anthropics/agentsmesh/backend/internal/service/admin"
	"github.com/gin-gonic/gin"
	"github.com/stretchr/testify/assert"
)

// =============================================================================
// Runner Error Path Tests
// =============================================================================

func TestRunnerHandler_DisableRunner_InvalidID(t *testing.T) {
	t.Run("should return 400 for invalid ID", func(t *testing.T) {
		db := newMockHandlerDB()

		svc := adminservice.NewService(db)
		handler := NewRunnerHandler(svc)

		w := httptest.NewRecorder()
		c := createAdminContext(w)
		c.Request = httptest.NewRequest("POST", "/runners/invalid/disable", nil)
		c.Params = gin.Params{{Key: "id", Value: "invalid"}}

		handler.DisableRunner(c)

		assert.Equal(t, http.StatusBadRequest, w.Code)
	})
}

func TestRunnerHandler_EnableRunner_NotFound(t *testing.T) {
	t.Run("should return 404 when runner not found", func(t *testing.T) {
		db := newMockHandlerDB()

		svc := adminservice.NewService(db)
		handler := NewRunnerHandler(svc)

		w := httptest.NewRecorder()
		c := createAdminContext(w)
		c.Request = httptest.NewRequest("POST", "/runners/999/enable", nil)
		c.Params = gin.Params{{Key: "id", Value: "999"}}

		handler.EnableRunner(c)

		assert.Equal(t, http.StatusNotFound, w.Code)
	})

	t.Run("should return 400 for invalid ID", func(t *testing.T) {
		db := newMockHandlerDB()

		svc := adminservice.NewService(db)
		handler := NewRunnerHandler(svc)

		w := httptest.NewRecorder()
		c := createAdminContext(w)
		c.Request = httptest.NewRequest("POST", "/runners/invalid/enable", nil)
		c.Params = gin.Params{{Key: "id", Value: "invalid"}}

		handler.EnableRunner(c)

		assert.Equal(t, http.StatusBadRequest, w.Code)
	})
}

func TestRunnerHandler_DeleteRunner_NotFound(t *testing.T) {
	t.Run("should return 404 when runner not found", func(t *testing.T) {
		db := newMockHandlerDB()

		svc := adminservice.NewService(db)
		handler := NewRunnerHandler(svc)

		w := httptest.NewRecorder()
		c := createAdminContext(w)
		c.Request = httptest.NewRequest("DELETE", "/runners/999", nil)
		c.Params = gin.Params{{Key: "id", Value: "999"}}

		handler.DeleteRunner(c)

		assert.Equal(t, http.StatusNotFound, w.Code)
	})

	t.Run("should return 400 for invalid ID", func(t *testing.T) {
		db := newMockHandlerDB()

		svc := adminservice.NewService(db)
		handler := NewRunnerHandler(svc)

		w := httptest.NewRecorder()
		c := createAdminContext(w)
		c.Request = httptest.NewRequest("DELETE", "/runners/invalid", nil)
		c.Params = gin.Params{{Key: "id", Value: "invalid"}}

		handler.DeleteRunner(c)

		assert.Equal(t, http.StatusBadRequest, w.Code)
	})
}

func TestRunnerHandler_GetRunner_InvalidID(t *testing.T) {
	t.Run("should return 400 for invalid ID", func(t *testing.T) {
		db := newMockHandlerDB()

		svc := adminservice.NewService(db)
		handler := NewRunnerHandler(svc)

		w := httptest.NewRecorder()
		c := createAdminContext(w)
		c.Request = httptest.NewRequest("GET", "/runners/invalid", nil)
		c.Params = gin.Params{{Key: "id", Value: "invalid"}}

		handler.GetRunner(c)

		assert.Equal(t, http.StatusBadRequest, w.Code)
	})
}

func TestRunnerHandler_GetRunner_OrgNotFound(t *testing.T) {
	t.Run("should return 200 when runner found but org not found", func(t *testing.T) {
		db := newMockHandlerDB()
		db.runners[1] = &runner.Runner{ID: 1, NodeID: "test-node", OrganizationID: 999}

		svc := adminservice.NewService(db)
		handler := NewRunnerHandler(svc)

		w := httptest.NewRecorder()
		c := createAdminContext(w)
		c.Request = httptest.NewRequest("GET", "/runners/1", nil)
		c.Params = gin.Params{{Key: "id", Value: "1"}}

		handler.GetRunner(c)

		assert.Equal(t, http.StatusOK, w.Code)
	})
}
