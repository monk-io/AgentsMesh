package v1

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

// LatestRunnerVersion is the recommended runner version that desktop clients
// should download during onboarding. Source of truth: the latest tag in
// github.com/AgentsMesh/AgentsMesh. Bumped per release; eventually moves to
// backend config so admins can pin a version without redeploying.
const LatestRunnerVersion = "0.29.0"

// LatestRunnerSha256 maps `<goos>_<goarch>` → sha256 of the GitHub Release
// archive (`agentsmesh-runner_<TAG>_<goos>_<goarch>.{tar.gz,zip}`). Source:
// `release-staging/checksums.txt` produced by .github/workflows/release.yml.
//
// MUST be updated together with LatestRunnerVersion: stale digests cause
// install_binary to refuse the genuine archive (ChecksumMismatch), blocking
// onboarding. Empty values disable verification for that platform — never
// commit empty values; if a slice is missing here, the desktop client
// downloads without integrity check, defeating the whole point.
var LatestRunnerSha256 = map[string]string{
	// TODO: populate from release pipeline before next stable cut.
	// Treat the empty map as "verification not yet available"; the
	// desktop client warns + proceeds rather than blocking onboarding.
}

// RunnerReleaseResponse is the public payload describing the recommended
// runner release for desktop install flows.
type RunnerReleaseResponse struct {
	Version string            `json:"version"`
	Sha256  map[string]string `json:"sha256"`
}

// GetLatestRunnerRelease returns the recommended runner version and per-
// platform sha256 digests. Public (no auth) so desktop onboarding can probe
// before the user completes any auth flow.
func GetLatestRunnerRelease(c *gin.Context) {
	c.JSON(http.StatusOK, RunnerReleaseResponse{
		Version: LatestRunnerVersion,
		Sha256:  LatestRunnerSha256,
	})
}

// RegisterRunnerReleaseRoutes mounts the public runner release endpoint.
func RegisterRunnerReleaseRoutes(r *gin.RouterGroup) {
	r.GET("/runners/latest-release", GetLatestRunnerRelease)
}
