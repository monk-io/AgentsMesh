package blockstore

// registerSystemSpecs seeds types that the platform itself writes or reads:
// user-defined schema (type_def), automation triggers (trigger_def), their
// event emissions (agent_event), and the richtext container backing
// ticket.content (document).
func registerSystemSpecs(m map[string]BlockTypeSpec) {
	m[BlockTypeTypeDef] = BlockTypeSpec{
		Type:        BlockTypeTypeDef,
		DefaultView: "list",
		// A new block type definition must identify which type it registers.
		// Other fields (default_view, required_data_key ...) are optional.
		RequiredDataKey: []string{"type_key"},
		AllowedChildren: nil,
	}
	m[BlockTypeTriggerDef] = BlockTypeSpec{
		Type:           BlockTypeTriggerDef,
		DefaultView:    "list",
		SupportedViews: []string{"list"},
		// data: { name, target_type, on, predicate?, action, enabled }
		// See service/blockstore/trigger_engine.go for semantics.
		RequiredDataKey: []string{"name", "target_type", "on", "action"},
		AllowedChildren: []string{},
	}
	m[BlockTypeAgentEvent] = BlockTypeSpec{
		Type:           BlockTypeAgentEvent,
		DefaultView:    "list",
		SupportedViews: []string{"list"},
		// data: { agent_slug, trigger_name, target_id?, target_type, op_kind,
		//         fired_at, consumed? }
		// Written by trigger_engine.fireAgentAction; consumed by agents via
		// memory.retrieve(type="agent_event") or subtree query.
		RequiredDataKey: []string{"agent_slug", "trigger_name", "op_kind", "fired_at"},
		AllowedChildren: []string{},
	}
	m[BlockTypeDocument] = BlockTypeSpec{
		Type:           BlockTypeDocument,
		DefaultView:    "document",
		SupportedViews: []string{"document"},
		// data: { blocknote_ast: [...] }
		// block.text holds the flattened plain-text for semantic search.
		AllowedChildren: []string{},
	}
}
