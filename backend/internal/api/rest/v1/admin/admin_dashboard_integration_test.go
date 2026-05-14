package admin

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/anthropics/agentsmesh/backend/internal/domain/user"
	"github.com/anthropics/agentsmesh/backend/internal/infra/database"
	adminservice "github.com/anthropics/agentsmesh/backend/internal/service/admin"
	"github.com/anthropics/agentsmesh/backend/internal/testkit"
	"github.com/gin-gonic/gin"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

// setupIntegrationDB creates a real SQLite DB wrapped in the database.DB interface.
func setupIntegrationDB(t *testing.T) database.DB {
	t.Helper()
	gormDB := testkit.SetupTestDB(t)
	return database.NewGormWrapper(gormDB)
}

// createAdminIntegrationContext builds a Gin context with admin user set.
func createAdminIntegrationContext(w *httptest.ResponseRecorder, adminID int64) *gin.Context {
	c, _ := gin.CreateTestContext(w)
	c.Set("admin_user_id", adminID)
	c.Set("admin_user", &user.User{ID: adminID, Email: "admin@test.com", IsSystemAdmin: true})
	return c
}

func TestAdminDashboard_Stats(t *testing.T) {
	db := setupIntegrationDB(t)
	gormDB := db.GormDB()

	adminID := testkit.CreateUser(t, gormDB, "admin@test.com", "admin")
	testkit.CreateUser(t, gormDB, "user1@test.com", "user1")
	testkit.CreateUser(t, gormDB, "user2@test.com", "user2")

	orgID := testkit.CreateOrg(t, gormDB, "org-alpha", adminID)
	testkit.CreateOrg(t, gormDB, "org-beta", adminID)

	testkit.CreateRunner(t, gormDB, orgID, "runner-node-1")

	svc := adminservice.NewService(db)
	handler := NewDashboardHandler(svc)

	w := httptest.NewRecorder()
	c := createAdminIntegrationContext(w, adminID)
	c.Request = httptest.NewRequest("GET", "/dashboard/stats", nil)

	handler.GetStats(c)

	assert.Equal(t, http.StatusOK, w.Code)

	var stats map[string]interface{}
	err := json.Unmarshal(w.Body.Bytes(), &stats)
	require.NoError(t, err)

	assert.Equal(t, float64(3), stats["total_users"])
	assert.Equal(t, float64(3), stats["active_users"])
	assert.Equal(t, float64(2), stats["total_organizations"])
	assert.Equal(t, float64(1), stats["total_runners"])
	assert.Equal(t, float64(1), stats["online_runners"])
}
