package blockstore

// bootstrapTypeSpecs is the built-in registry seeded into every workspace.
// Users and Agents may override any entry by writing a block_type_def block
// whose data.type_key matches; see service/blockstore/type_resolver.go.
//
// Composed from per-category register* functions so no single file holds the
// entire 28-type literal. Order inside buildBootstrapSpecs is load order —
// later registrations win if two files accidentally claim the same key
// (currently none do; the tests in domain_test.go cover regressions).
var bootstrapTypeSpecs = buildBootstrapSpecs()

func buildBootstrapSpecs() map[string]BlockTypeSpec {
	m := make(map[string]BlockTypeSpec, 32)
	registerContentSpecs(m)
	registerMediaSpecs(m)
	registerSystemSpecs(m)
	return m
}

// LookupTypeSpec returns the bootstrap spec for a built-in block type.
// Prefer TypeResolver.Resolve when a DB handle is available — it knows about
// runtime-registered types.
func LookupTypeSpec(t string) (BlockTypeSpec, bool) {
	spec, ok := bootstrapTypeSpecs[t]
	return spec, ok
}

// BootstrapBlockTypes returns the list of built-in type keys in stable order.
// Consumers that rely on a deterministic iteration (e.g. MCP tool discovery
// assembling a schema catalog) use this instead of ranging the map.
func BootstrapBlockTypes() []string {
	return []string{
		BlockTypePage, BlockTypeParagraph, BlockTypeTask,
		BlockTypeList, BlockTypeView, BlockTypeTypeDef, BlockTypeComment,
		BlockTypeHeading, BlockTypeDivider, BlockTypeCode,
		BlockTypeQuote, BlockTypeCallout,
		BlockTypeBulletedListItem, BlockTypeNumberedListItem, BlockTypeToggle,
		BlockTypeLinkToPage,
		BlockTypeImage, BlockTypeFile, BlockTypeVideo,
		BlockTypeEmbed, BlockTypeBookmark,
		BlockTypeAudio, BlockTypeEquation, BlockTypeChart, BlockTypeSyncedBlock,
		BlockTypeTable, BlockTypeMention,
		BlockTypeTriggerDef, BlockTypeAgentEvent, BlockTypeDocument,
		BlockTypeColumnList, BlockTypeColumn,
	}
}
