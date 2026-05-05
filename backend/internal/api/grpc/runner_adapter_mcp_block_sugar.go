package grpc

import (
	"context"

	"github.com/anthropics/agentsmesh/backend/internal/domain/blockstore"
	"github.com/anthropics/agentsmesh/backend/internal/middleware"
)

// indicator.define is a sugared createBlock that wraps the arguments into a
// block_type_def data payload. Kept as a separate tool in the MCP surface so
// agents have a clearer mental model ("register a schema" vs "create a
// block of type X"); semantically it is a createBlock writing exactly one
// row in the type registry.
func (a *GRPCRunnerAdapter) mcpIndicatorDefine(ctx context.Context, tc *middleware.TenantContext, payload []byte) (interface{}, *mcpError) {
	var params struct {
		WorkspaceID    string         `json:"workspace_id"`
		IdempotencyKey string         `json:"idempotency_key,omitempty"`
		Arguments      map[string]any `json:"arguments"`
	}
	if err := unmarshalPayload(payload, &params); err != nil {
		return nil, err
	}
	typeKey, _ := params.Arguments["type_key"].(string)
	if typeKey == "" {
		return nil, newMcpError(400, "type_key is required")
	}
	// Clone so the blockstoreservice doesn't mutate the agent-supplied map.
	data := make(map[string]any, len(params.Arguments))
	for k, v := range params.Arguments {
		data[k] = v
	}
	return a.applyOneWithParams(ctx, tc, blockstore.OpCreateBlock, applyOpsPayload{
		WorkspaceID:    params.WorkspaceID,
		IdempotencyKey: params.IdempotencyKey,
		Payload: map[string]any{
			"type": blockstore.BlockTypeTypeDef,
			"data": data,
			"text": typeKey,
		},
	})
}

// trigger.define registers a trigger_def block. Defaults `enabled=true` when
// unset so the common "register + immediately active" path skips a field.
// SSRF guard lives in blockstore service layer (webhook_url_guard.go); any
// action.url pointing at loopback/RFC1918/etc. is rejected inside
// applyCreateBlock → validateTriggerDefData.
func (a *GRPCRunnerAdapter) mcpTriggerDefine(ctx context.Context, tc *middleware.TenantContext, payload []byte) (interface{}, *mcpError) {
	var params struct {
		WorkspaceID    string         `json:"workspace_id"`
		IdempotencyKey string         `json:"idempotency_key,omitempty"`
		Arguments      map[string]any `json:"arguments"`
	}
	if err := unmarshalPayload(payload, &params); err != nil {
		return nil, err
	}
	name, _ := params.Arguments["name"].(string)
	if name == "" {
		return nil, newMcpError(400, "name is required")
	}
	data := make(map[string]any, len(params.Arguments))
	for k, v := range params.Arguments {
		data[k] = v
	}
	if _, has := data["enabled"]; !has {
		data["enabled"] = true
	}
	return a.applyOneWithParams(ctx, tc, blockstore.OpCreateBlock, applyOpsPayload{
		WorkspaceID:    params.WorkspaceID,
		IdempotencyKey: params.IdempotencyKey,
		Payload: map[string]any{
			"type": blockstore.BlockTypeTriggerDef,
			"data": data,
			"text": name,
		},
	})
}
