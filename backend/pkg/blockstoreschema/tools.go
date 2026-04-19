// Package blockstoreschema exposes Block Store op definitions as JSON schemas
// suitable for direct use as Anthropic / OpenAI "tool" definitions. Agents
// receive one tool per op kind; the union is everything the service accepts.
package blockstoreschema

import (
	"github.com/anthropics/agentsmesh/backend/internal/domain/blockstore"
)

// Tool mirrors the structure that LLM providers expect for tool-use. Kept
// provider-neutral; adapters wrap this into Anthropic / OpenAI formats at
// call sites.
type Tool struct {
	Name        string         `json:"name"`
	Description string         `json:"description"`
	InputSchema map[string]any `json:"input_schema"`
}

// AllTools returns the static Phase 1 tool list using bootstrap types only.
// Use BuildToolsWithTypes to include runtime-registered block_type_def types.
func AllTools() []Tool {
	return BuildToolsWithTypes(blockstore.BootstrapBlockTypes())
}

// BuildToolsWithTypes returns the tool list with a caller-supplied list of
// block type keys. Callers that have a workspace context should pass the
// resolver's full (bootstrap + dynamic) type set.
func BuildToolsWithTypes(typeKeys []string) []Tool {
	return []Tool{
		createBlockTool(typeKeys),
		updateBlockTool(),
		deleteBlockTool(),
		addRefTool(),
		removeRefTool(),
		updateRefTool(),
		memoryRetrieveTool(),
		defineIndicatorTool(),
		defineTriggerTool(),
	}
}

// ToolMemoryRetrieve is the MCP tool name for Agent long-term memory. Kept as
// a constant so handlers can special-case it without string-matching.
const ToolMemoryRetrieve = "memory.retrieve"

// ToolDefineIndicator registers (or revises) a column-driven block type
// (Tier 1 "indicator" system). Equivalent to writing a block_type_def block
// by hand, but the distinct name gives Agents a clearer mental model and
// lets the handler layer validate the column schema before persisting.
const ToolDefineIndicator = "indicator.define"

// ToolDefineTrigger registers a reactive rule: "when op X happens to type Y
// with predicate Z, fire action". Stored as a trigger_def block so the
// workflow audit trail, ACL, and op log all apply uniformly.
const ToolDefineTrigger = "trigger.define"

func schemaObject(properties map[string]any, required []string) map[string]any {
	return map[string]any{
		"type":       "object",
		"properties": properties,
		"required":   required,
	}
}
