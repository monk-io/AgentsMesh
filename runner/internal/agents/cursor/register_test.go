package cursor

import (
	"testing"

	"github.com/anthropics/agentsmesh/runner/internal/agentkit"
	"github.com/anthropics/agentsmesh/runner/internal/tokenusage"
	"github.com/stretchr/testify/assert"
)

func TestCursorRegistered(t *testing.T) {
	// Opt-out covers BOTH the DB slug and the runtime launch_command key.
	assert.True(t, tokenusage.IsParserOptOut("cursor-cli"), "slug cursor-cli should be opt-out from token-usage fixture contract")
	assert.True(t, tokenusage.IsParserOptOut("cursor-agent"), "launch_command cursor-agent (the runtime token-collection key) should be opt-out")
	assert.Nil(t, tokenusage.GetParser("cursor-cli"), "opt-out agents must not register a parser (slug key)")
	assert.Nil(t, tokenusage.GetParser("cursor-agent"), "opt-out agents must not register a parser (runtime launch_command key)")

	assert.True(t, agentkit.IsAgentProcess("cursor-agent"), "process name cursor-agent must be registered")
	assert.False(t, agentkit.IsAgentProcess("cursor"), "bare 'cursor' must NOT be registered (collides with Cursor IDE)")
	assert.False(t, agentkit.IsAgentProcess("cursor-cli"), "the DB slug must NOT be registered as a process name (the binary is cursor-agent)")
}
