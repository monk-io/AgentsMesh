package v1

import (
	"strconv"

	"github.com/anthropics/agentsmesh/backend/pkg/apierr"
	"github.com/gin-gonic/gin"
)

// CreateEnvBundleRequest is the POST payload for /env-bundles.
type CreateEnvBundleRequest struct {
	AgentSlug   *string           `json:"agent_slug"`
	Name        string            `json:"name" binding:"required,max=100"`
	Description *string           `json:"description"`
	Kind        string            `json:"kind" binding:"required,max=32"`
	KindPrimary bool              `json:"kind_primary"`
	Data        map[string]string `json:"data"`
}

// UpdateEnvBundleRequest is the PUT payload for /env-bundles/:id. Data has
// three states on the wire:
//   - field absent       → nil → "leave data unchanged"
//   - `"data": {}`        → &empty → "clear all keys"
//   - `"data": {"k":...}` → &non-empty → "replace stored values"
type UpdateEnvBundleRequest struct {
	Name        *string            `json:"name"`
	Description *string            `json:"description"`
	Kind        *string            `json:"kind"`
	KindPrimary *bool              `json:"kind_primary"`
	Data        *map[string]string `json:"data"`
	IsActive    *bool              `json:"is_active"`
}

// parseInt64Param extracts an int64 path parameter and writes a 400 reply
// when malformed. Returns the value and a nil error on success.
func parseInt64Param(c *gin.Context, key string) (int64, error) {
	id, err := strconv.ParseInt(c.Param(key), 10, 64)
	if err != nil {
		apierr.InvalidInput(c, "Invalid "+key)
		return 0, err
	}
	return id, nil
}
