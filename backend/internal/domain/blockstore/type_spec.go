package blockstore

// BlockTypeSpec describes an accepted block type. In Phase 3 the Service layer
// resolves specs from persisted block_type_def blocks first, falling back to
// this static bootstrap registry when no DB row exists for the given type.
//
// Phase 5 (Tier 1 of the indicator system) elevates `Columns` to a first-class
// concept: any type that declares Columns is renderable by the generic
// RecordEditor and queryable by schema-driven views (kanban group_by /
// table columns / gallery card layout). Legacy `RequiredDataKey` remains for
// compatibility with bootstrap types whose UI is still hardcoded (task / page).
type BlockTypeSpec struct {
	Type            string       // e.g. "task"
	Revision        int          // incremented when the spec shape changes (dynamic specs only)
	Label           string       // human-readable ("OKR", "Bug"); optional, defaults to Type
	Description     string       // one-line intent shown in slash menu / MCP descriptions
	DefaultView     string       // e.g. "document"
	SupportedViews  []string     // e.g. ["document", "list", "kanban"]
	RequiredDataKey []string     // legacy: presence-only check; empty for Columns-driven types
	AllowedChildren []string     // empty = any child block type; otherwise whitelist
	Columns         []ColumnSpec // Tier 1: schema-driven record fields

	// EnumValues constrains specific data keys to an allowed set of string
	// values. Applied by ValidateRecord in addition to RequiredDataKey/Columns.
	// Used for bootstrap types whose shape isn't Tier-1/column-driven but still
	// needs value validation (e.g. chart.data.type ∈ {bar, line, pie, ...}).
	EnumValues map[string][]string

	// NonEmptyArrayKeys lists data keys that must be JSON arrays with at least
	// one element when present. Paired with RequiredDataKey for keys that also
	// must exist; stand-alone when emptiness is the only concern. Guards against
	// `{type:"bar", series: []}` being persisted and then rendered as a silent
	// placeholder.
	NonEmptyArrayKeys []string
}

// ColumnType enumerates the first-class field kinds the generic RecordEditor
// knows how to render. Type-specific rendering (marks, calculated subfields)
// is out of scope; custom UIs can still ship as dedicated renderers bypassing
// RecordEditor (page / task / document are examples).
type ColumnType string

const (
	ColumnTypeText        ColumnType = "text"
	ColumnTypeNumber      ColumnType = "number"
	ColumnTypeBoolean     ColumnType = "boolean"
	ColumnTypeSelect      ColumnType = "select"
	ColumnTypeMultiSelect ColumnType = "multi_select"
	ColumnTypeDate        ColumnType = "date"
	ColumnTypeURL         ColumnType = "url"
	ColumnTypeUser        ColumnType = "user"
	ColumnTypeBlockRef    ColumnType = "block_ref"
)

// ColumnSpec defines one field on an indicator type. Stored inside
// block_type_def.data.columns; the type resolver hydrates ColumnSpec arrays
// onto BlockTypeSpec at read time.
type ColumnSpec struct {
	Key         string         `json:"key"`
	Label       string         `json:"label,omitempty"`
	Type        ColumnType     `json:"type"`
	Required    bool           `json:"required,omitempty"`
	Default     any            `json:"default,omitempty"`
	Options     []SelectOption `json:"options,omitempty"`
	Description string         `json:"description,omitempty"`

	// Computed expression. When non-empty the column is read-only in the UI
	// and its value is re-derived at render time from the record's other
	// fields. See pkg/blockstore/expr for the supported syntax (basic
	// arithmetic + column references, intentionally safe).
	Computed string `json:"computed,omitempty"`

	// Deprecated flips this column to "hidden but preserved": renderers skip
	// it, validators allow missing values, but existing data survives. Set
	// when evolving a schema — prefer deprecation to outright deletion.
	Deprecated bool `json:"deprecated,omitempty"`
}

// SelectOption is one choice in a select / multi_select column. Color gives
// the renderer an optional style hint; callers are free to ignore it.
type SelectOption struct {
	Value string `json:"value"`
	Label string `json:"label,omitempty"`
	Color string `json:"color,omitempty"`
}

// IsColumnsDriven reports whether this spec declares a full Columns schema.
// Columns-driven types get generic RecordEditor rendering; others fall back
// to the hardcoded renderer registry.
func (spec BlockTypeSpec) IsColumnsDriven() bool {
	return len(spec.Columns) > 0
}

// IsChildAllowed reports whether the given child type may be nested under
// this spec. Empty AllowedChildren means "any".
func (spec BlockTypeSpec) IsChildAllowed(child string) bool {
	if len(spec.AllowedChildren) == 0 {
		return true
	}
	for _, allowed := range spec.AllowedChildren {
		if allowed == child {
			return true
		}
	}
	return false
}

// RequiredKeys returns the union of RequiredDataKey and any column marked
// Required. Used by ValidateRequiredFields so Tier 1 types don't need to
// duplicate their required list in two places. Computed and deprecated
// columns are excluded — the caller never writes them directly.
func (spec BlockTypeSpec) RequiredKeys() []string {
	out := make([]string, 0, len(spec.RequiredDataKey)+len(spec.Columns))
	seen := make(map[string]bool, len(out))
	for _, k := range spec.RequiredDataKey {
		if !seen[k] {
			seen[k] = true
			out = append(out, k)
		}
	}
	for _, c := range spec.Columns {
		if c.Deprecated || c.Computed != "" {
			continue
		}
		if c.Required && !seen[c.Key] {
			seen[c.Key] = true
			out = append(out, c.Key)
		}
	}
	return out
}
