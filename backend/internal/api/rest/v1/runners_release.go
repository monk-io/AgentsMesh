package v1

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

// LatestRunnerVersion is the recommended runner version that desktop clients
// should download during onboarding. Bumped per release; eventually moves to
// backend config so admins can pin a version without redeploying.
const LatestRunnerVersion = "0.4.7"

// RunnerReleaseResponse is the public payload describing the recommended
// runner release for desktop install flows.
type RunnerReleaseResponse struct {
	Version string `json:"version"`
}

// GetLatestRunnerRelease returns the recommended runner version. Public
// (no auth) so desktop onboarding can probe the value before the user
// completes any auth flow.
func GetLatestRunnerRelease(c *gin.Context) {
	c.JSON(http.StatusOK, RunnerReleaseResponse{Version: LatestRunnerVersion})
}

// RegisterRunnerReleaseRoutes mounts the public runner release endpoint.
func RegisterRunnerReleaseRoutes(r *gin.RouterGroup) {
	r.GET("/runners/latest-release", GetLatestRunnerRelease)
}
