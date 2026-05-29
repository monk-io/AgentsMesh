-- 000157_add_cursor_cli_agent.up.sql
-- Register Cursor CLI (cursor-agent) as a builtin agent.
--
-- Cursor CLI is Anysphere's terminal coding agent. The binary on disk is
-- `cursor-agent` (installed via curl https://cursor.com/install to
-- ~/.local/bin/cursor-agent). The DB `slug` keeps the `-cli` suffix to match
-- the claude-code / codex-cli / gemini-cli naming convention, but the
-- AgentFile's AGENT/EXECUTABLE fields MUST be the actual binary name —
-- agentfile/eval/eval_decl.go:10 wires AGENT directly into LaunchCommand,
-- which the runner exec()s. Compare migration 000103: slug='claude-code' but
-- AGENT=claude (the binary). Using AGENT=cursor-cli here would cause every
-- pod to fail with ENOENT.
--
-- First-pass integration is PTY-only — ACP adapter is deferred (see plan:
-- hashed-strolling-moth.md, "Out of scope"). MCP is intentionally NOT declared
-- (no `MCP ON`): cursor-agent uses its own ~/.cursor/mcp.json format and we do
-- not inject it, so declaring MCP would set MCPEnabled=true on BuildResult
-- without any config landing in the sandbox — surfacing a capability we don't
-- deliver.
--
-- CURSOR_API_KEY is declared as an OPTIONAL secret: cursor-agent authenticates
-- via `cursor-agent login` OAuth (token in HOME, inherited by the pod) by
-- default, but also honors CURSOR_API_KEY for headless/key-based auth. The ENV
-- declaration is pure schema metadata (agentfile/eval/eval_decl.go skips
-- `ENV X SECRET OPTIONAL` at eval time); it lets the frontend render a curated
-- credential field, mirroring gemini-cli's GOOGLE_API_KEY.

INSERT INTO agents (slug, name, launch_command, executable, is_builtin, is_active, supported_modes, agentfile_source)
VALUES ('cursor-cli', 'Cursor CLI', 'cursor-agent', 'cursor-agent', true, true, 'pty',
  E'# === Identity ===\nAGENT cursor-agent\nEXECUTABLE cursor-agent\n\n# === Mode ===\nMODE pty\n\n# === Environment ===\nENV CURSOR_API_KEY SECRET OPTIONAL\n\n# === Prompt ===\nPROMPT_POSITION prepend\n');
