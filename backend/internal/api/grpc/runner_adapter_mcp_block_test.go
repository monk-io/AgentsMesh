package grpc

import (
	"context"
	"errors"
	"testing"

	"github.com/stretchr/testify/assert"

	"github.com/anthropics/agentsmesh/backend/internal/domain/blockstore"
	"github.com/anthropics/agentsmesh/backend/internal/middleware"
)

// Pure unit coverage for the two most-likely-to-regress bits of the
// Block Store gRPC dispatch: the Actor identity derivation (agent vs user
// fallback) and the error → mcpError code mapping. End-to-end coverage
// comes from the E2E security-guards / ref-invariants / block-crud specs
// once Runner-side MCP bridging lands (see http_tools_block_test.go).

func TestActorFromTenant_AgentWhenPodIDPresent(t *testing.T) {
	podID := int64(42)
	tc := &middleware.TenantContext{
		OrganizationID: 7,
		UserID:         100,
		PodID:          &podID,
	}
	a := actorFromTenant(tc)
	assert.Equal(t, int64(7), a.OrgID)
	// The permission-bearing field: UserID must reflect the pod creator so
	// ACL checks and block.CreatedBy resolve against a real human — the
	// "权限跟着人走" rule. Agents have no standalone principal.
	assert.Equal(t, int64(100), a.UserID,
		"UserID must be the pod creator, not the pod id — agents borrow human permissions")
	// Audit-only fields describe origin without changing attribution.
	assert.Equal(t, blockstore.ActorAgent, a.ActorType)
	assert.Equal(t, podID, a.ActorID, "ActorID is audit-only; reflects the pod, not the user")
}

func TestActorFromTenant_UserFallbackWhenPodIDAbsent(t *testing.T) {
	tc := &middleware.TenantContext{
		OrganizationID: 7,
		UserID:         100,
		// PodID nil — this is the REST path or a misconfigured gRPC call.
	}
	a := actorFromTenant(tc)
	assert.Equal(t, blockstore.ActorUser, a.ActorType)
	assert.Equal(t, tc.UserID, a.ActorID,
		"fallback must still give a meaningful ActorID for the audit trail")
}

func TestBlockstoreErrToMcp_Mapping(t *testing.T) {
	cases := []struct {
		name     string
		err      error
		wantCode int32
	}{
		{"workspace not found → 404", blockstore.ErrWorkspaceNotFound, 404},
		{"block not found → 404", blockstore.ErrBlockNotFound, 404},
		{"ref not found → 404", blockstore.ErrRefNotFound, 404},
		{"org mismatch → 403", blockstore.ErrOrgMismatch, 403},
		{"forbidden → 403", blockstore.ErrBlockForbidden, 403},
		{"single-parent nest → 409", blockstore.ErrSingleNestParent, 409},
		{"nest cycle → 409", blockstore.ErrNestCycle, 409},
		{"stale update → 409", blockstore.ErrStaleUpdate, 409},
		{"unknown block type → 400", blockstore.ErrUnknownBlockType, 400},
		{"missing required key → 400", blockstore.ErrMissingRequiredKey, 400},
		{"column value invalid → 400", blockstore.ErrColumnValueInvalid, 400},
		{"apply ops empty → 400", blockstore.ErrApplyOpsEmpty, 400},
		{"arbitrary error → 500", errors.New("something unexpected"), 500},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := blockstoreErrToMcp(tc.err)
			if got == nil {
				t.Fatalf("blockstoreErrToMcp returned nil for %v", tc.err)
			}
			assert.Equal(t, tc.wantCode, got.code)
		})
	}
}

func TestApplyOneWithParams_NilServiceReturns501(t *testing.T) {
	a := &GRPCRunnerAdapter{}
	_, merr := a.applyOneWithParams(context.Background(), &middleware.TenantContext{}, blockstore.OpCreateBlock, applyOpsPayload{
		WorkspaceID: "ws",
		Payload:     map[string]any{},
	})
	if merr == nil || merr.code != 501 {
		t.Fatalf("expected 501 when blockstoreService nil, got %+v", merr)
	}
}

// Deeper dispatch tests (workspace missing → 400, valid payload → service
// call) rely on a live blockstore.Service with DB fixtures and are covered
// by the E2E suite (schema-validation.spec.ts, block-crud.spec.ts,
// security-guards.spec.ts) rather than duplicated here. The pure helpers
// above are where regressions will actually surface first.
