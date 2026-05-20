-- 000150_e2e_echo_print_env_on_startup.up.sql
-- Make e2e-echo write whitelisted env vars to a sandbox file on startup so
-- EnvBundle e2e tests can verify ENV injection end-to-end (Settings UI →
-- Pod create dialog → backend agentfile eval → runner → child process env).
--
-- Why a file (not stdout): the runner's PTY → relay stream is asynchronous
-- and lossy for daemon-managed pods (we observed `total_reads=0` on the
-- master FD when the child exits before its output makes it through the
-- daemon IPC); writing to disk gives the e2e a deterministic, race-free
-- artifact to read via `docker exec cat`.
--
-- The agent still prints "ready" first (mcp-e2e fixture detects liveness
-- via input/echo round-trip, not via "ready" string match — see
-- tests/mcp-e2e/suites/pod_interaction_test.go), then continues into the
-- existing read-and-echo loop unchanged.
BEGIN;

UPDATE agents
SET agentfile_source = E'AGENT bash\nEXECUTABLE bash\n\nMODE pty\nMCP ON\n\n# Build logic: dump whitelisted env to disk + run the echo loop.\n# - Prints "ready" first (mcp-e2e liveness anchor).\n# - Dumps env entries matching E2E_TEST_, ANTHROPIC_, CLAUDE_ prefixes to\n#   /tmp/e2e-echo-env-dump-$$ so the EnvBundle e2e can verify injection\n#   via `docker exec` without depending on PTY streaming.\n# - Reads stdin and echoes "got: <line>" (existing round-trip behavior).\narg "-c" "echo ready; env | grep -E ''^(E2E_TEST_|ANTHROPIC_|CLAUDE_)'' | sort > /tmp/e2e-echo-env-dump-$$; while IFS= read -r line; do echo \\"got: $line\\"; done"\n'
WHERE slug = 'e2e-echo';

COMMIT;
