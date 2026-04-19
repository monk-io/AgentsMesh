package blockstoreservice

import (
	"encoding/json"

	"github.com/anthropics/agentsmesh/backend/internal/domain/blockstore"
)

// BlockACL mirrors the shape stored inside block.meta.acl. A nil / empty ACL
// object falls back to workspace-level visibility (the Phase 1 default).
//
//	{
//	  "visibility":    "workspace" | "private" | "org",
//	  "allowed_users": [1, 2, 3]
//	}
type BlockACL struct {
	Visibility   string  `json:"visibility"`
	AllowedUsers []int64 `json:"allowed_users"`
}

// extractACL pulls an ACL from block.meta, returning a zero value when absent.
func extractACL(meta blockstore.JSONMap) BlockACL {
	raw, ok := meta["acl"]
	if !ok {
		return BlockACL{}
	}
	buf, err := json.Marshal(raw)
	if err != nil {
		return BlockACL{}
	}
	var out BlockACL
	_ = json.Unmarshal(buf, &out)
	return out
}

// allows reports whether actor has access to a block under the given acl.
// The ACL only tightens access — it cannot escalate beyond the org-level
// membership that the surrounding TenantMiddleware already verified.
func (acl BlockACL) allows(actorUserID, createdBy int64) bool {
	switch acl.Visibility {
	case "", "workspace", "org":
		return true // default: any authenticated org member
	case "private":
		if actorUserID == createdBy {
			return true
		}
		for _, u := range acl.AllowedUsers {
			if u == actorUserID {
				return true
			}
		}
		return false
	default:
		// Unknown visibility values fail CLOSED. A typo or corrupt meta.acl
		// row must not silently leak a block the author meant to lock down;
		// the safe default is to deny until the value is one of the known
		// set ("", "workspace", "org", "private").
		return false
	}
}
