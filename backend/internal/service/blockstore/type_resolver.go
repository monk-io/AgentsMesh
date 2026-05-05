package blockstoreservice

import (
	"context"
	"encoding/json"

	"github.com/anthropics/agentsmesh/backend/internal/domain/blockstore"
	"github.com/google/uuid"
)

// resolveTypeSpecInTx reads block_type_def rows through the active TxWriter so
// definitions inserted earlier in the same ApplyOps batch are visible to
// subsequent ops. For lookups *outside* a transaction, use
// Service.listAllTypes or the domain-layer blockstore.LookupTypeSpec directly.
func (s *Service) resolveTypeSpecInTx(
	ctx context.Context,
	tx blockstore.TxWriter,
	typeKey string,
) (blockstore.BlockTypeSpec, bool) {
	defs, err := tx.ListTypeDefs(ctx)
	if err == nil {
		if spec, ok := pickLatestTypeDef(defs, typeKey); ok {
			return spec, true
		}
	}
	return blockstore.LookupTypeSpec(typeKey)
}

func pickLatestTypeDef(blocks []*blockstore.Block, typeKey string) (blockstore.BlockTypeSpec, bool) {
	var best blockstore.BlockTypeSpec
	var found bool
	for _, b := range blocks {
		spec, ok := decodeTypeDef(b.Data)
		if !ok || spec.Type != typeKey {
			continue
		}
		if !found || spec.Revision > best.Revision {
			best = spec
			found = true
		}
	}
	return best, found
}

func decodeTypeDef(data blockstore.JSONMap) (blockstore.BlockTypeSpec, bool) {
	raw, err := json.Marshal(data)
	if err != nil {
		return blockstore.BlockTypeSpec{}, false
	}
	var shape struct {
		TypeKey         string                  `json:"type_key"`
		Revision        int                     `json:"revision"`
		Label           string                  `json:"label"`
		Description     string                  `json:"description"`
		DefaultView     string                  `json:"default_view"`
		SupportedViews  []string                `json:"supported_views"`
		RequiredDataKey []string                `json:"required_data_key"`
		AllowedChildren []string                `json:"allowed_children"`
		Columns         []blockstore.ColumnSpec `json:"columns"`
	}
	if err := json.Unmarshal(raw, &shape); err != nil {
		return blockstore.BlockTypeSpec{}, false
	}
	if shape.TypeKey == "" {
		return blockstore.BlockTypeSpec{}, false
	}
	return blockstore.BlockTypeSpec{
		Type:            shape.TypeKey,
		Revision:        shape.Revision,
		Label:           shape.Label,
		Description:     shape.Description,
		DefaultView:     shape.DefaultView,
		SupportedViews:  shape.SupportedViews,
		RequiredDataKey: shape.RequiredDataKey,
		AllowedChildren: shape.AllowedChildren,
		Columns:         shape.Columns,
	}, true
}

// listAllTypes returns every type registered for a workspace: bootstrap types
// unioned with the latest dynamic block_type_def entries.
// Used by the MCP tools endpoint (tx-external) to expose the live capability set.
func (s *Service) listAllTypes(
	ctx context.Context,
	workspaceID uuid.UUID,
) []blockstore.BlockTypeSpec {
	seen := make(map[string]blockstore.BlockTypeSpec)
	for _, key := range blockstore.BootstrapBlockTypes() {
		if spec, ok := blockstore.LookupTypeSpec(key); ok {
			seen[key] = spec
		}
	}
	def := blockstore.BlockTypeTypeDef
	blocks, _, err := s.repo.ListBlocks(ctx, blockstore.BlockFilter{
		WorkspaceID: workspaceID,
		Type:        &def,
	})
	if err == nil {
		for _, b := range blocks {
			spec, ok := decodeTypeDef(b.Data)
			if !ok {
				continue
			}
			if existing, exists := seen[spec.Type]; exists && existing.Revision >= spec.Revision {
				continue
			}
			seen[spec.Type] = spec
		}
	}
	out := make([]blockstore.BlockTypeSpec, 0, len(seen))
	for _, spec := range seen {
		out = append(out, spec)
	}
	return out
}
