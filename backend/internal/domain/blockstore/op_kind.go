package blockstore

// Op kinds accepted by ApplyOps. The initial set is deliberately minimal —
// any business action decomposes to this set of primitive operations.
const (
	OpCreateBlock = "createBlock"
	OpUpdateBlock = "updateBlock"
	OpDeleteBlock = "deleteBlock"
	OpAddRef      = "addRef"
	OpRemoveRef   = "removeRef"
	OpUpdateRef   = "updateRef"
)

// Rel kinds recognized by the server in Phase 1.
// `rel` is extensible; unknown rels are accepted for storage but receive no
// server-side semantics (no uniqueness, no ordering guarantee, no cycle check).
const (
	RelNest        = "nest"
	RelMention     = "mention"
	RelEmbed       = "embed"
	RelDependsOn   = "depends_on"
	RelDerivedFrom = "derived_from"
	RelTag         = "tag"
	RelCommentsOn  = "comments_on" // Phase 4: a comment block anchored on another block
)

// Block types registered at bootstrap time. Additional types can be defined
// at runtime by writing blocks of type BlockTypeTypeDef — the service layer
// falls back to this static registry only when no matching DB row exists.
const (
	BlockTypePage      = "page"
	BlockTypeParagraph = "paragraph"
	BlockTypeTask      = "task"
	BlockTypeList      = "list"
	BlockTypeView      = "view"
	BlockTypeTypeDef   = "block_type_def"
	BlockTypeComment   = "comment"

	// Rich document blocks — day-to-day "Notion parity" set. Renderer lives
	// in the web package; data shape is documented on bootstrapTypeSpecs.
	BlockTypeHeading            = "heading"
	BlockTypeDivider            = "divider"
	BlockTypeCode               = "code"
	BlockTypeQuote              = "quote"
	BlockTypeCallout            = "callout"
	BlockTypeBulletedListItem   = "bulleted_list_item"
	BlockTypeNumberedListItem   = "numbered_list_item"
	BlockTypeToggle             = "toggle"
	BlockTypeLinkToPage         = "link_to_page"
	BlockTypeImage              = "image"
	BlockTypeFile               = "file"
	BlockTypeVideo              = "video"
	BlockTypeEmbed              = "embed"
	BlockTypeBookmark           = "bookmark"
	BlockTypeAudio              = "audio"
	BlockTypeEquation           = "equation"
	BlockTypeChart              = "chart"
	BlockTypeSyncedBlock        = "synced_block"
	BlockTypeTable              = "table"
	BlockTypeMention            = "mention"

	// Phase 5: structured rich-text document backed by BlockNote. Holds the
	// full BlockNote AST in data.blocknote_ast plus a flattened plain text
	// mirror in block.text for search / embeddings.
	BlockTypeDocument = "document"

	// Multi-column layout. column_list is the horizontal container; column
	// is a vertical slot inside. Regular blocks nest under column as usual.
	// Two new container types, no new rel kind.
	BlockTypeColumnList = "column_list"
	BlockTypeColumn     = "column"

	// Tier 3: Agent reactive loop. trigger_def describes "fire X when Y
	// happens to type Z" — stored as a block so it's editable, auditable,
	// and subject to the same ACL / workspace isolation as everything else.
	BlockTypeTriggerDef = "trigger_def"

	// agent_event is written by the trigger engine when action.kind="agent"
	// fires. Agents consume these by calling memory.retrieve with
	// type="agent_event" or listing them via /blocks/workspaces/:ws/subtree.
	// Fields: {agent_slug, trigger_name, target_id, target_type, op_kind,
	//          fired_at, consumed? bool}.
	BlockTypeAgentEvent = "agent_event"
)

// AllOpKinds enumerates accepted ops; used by validators and schema exporters.
func AllOpKinds() []string {
	return []string{
		OpCreateBlock, OpUpdateBlock, OpDeleteBlock,
		OpAddRef, OpRemoveRef, OpUpdateRef,
	}
}

// OrderedRels reports whether a rel value carries meaningful `order_key`.
func IsOrderedRel(rel string) bool {
	return rel == RelNest
}

// UniqueParentRels reports whether a rel enforces the single-parent invariant
// (one `to_id` may have at most one ref with this rel).
func IsUniqueParentRel(rel string) bool {
	return rel == RelNest
}
