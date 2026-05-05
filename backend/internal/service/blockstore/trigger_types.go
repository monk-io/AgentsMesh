package blockstoreservice

import "github.com/anthropics/agentsmesh/backend/internal/domain/blockstore"

// TriggerDef is the decoded shape of a BlockTypeTriggerDef block's data.
// Kept loose — the service layer validates required fields at fire-time so
// malformed trigger rows degrade gracefully instead of crashing ApplyOps.
type TriggerDef struct {
	Name       string             `json:"name"`
	TargetType string             `json:"target_type"`
	On         string             `json:"on"` // "create" | "update" | "delete"
	Predicate  string             `json:"predicate,omitempty"`
	Action     TriggerAction      `json:"action"`
	Enabled    bool               `json:"enabled"`
	Meta       blockstore.JSONMap `json:"-"`
	// CreatedBy is the user_id of whoever wrote this trigger_def block.
	// Because permission in this system flows "through to the human who
	// created the pod/agent/trigger" (权限跟着人走), every side-effect fired
	// by this trigger — notably agent_event writes — must be attributed to
	// CreatedBy so ACL checks work against a real user.
	CreatedBy int64 `json:"-"`
}

type TriggerAction struct {
	Kind      string            `json:"kind"` // "webhook" | "agent"
	URL       string            `json:"url,omitempty"`
	AgentSlug string            `json:"agent_slug,omitempty"`
	Headers   map[string]string `json:"headers,omitempty"`
}
