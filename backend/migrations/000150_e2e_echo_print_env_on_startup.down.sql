-- 000150_e2e_echo_print_env_on_startup.down.sql
-- Revert e2e-echo to the original "ready + echo loop" without env dump.
BEGIN;

UPDATE agents
SET agentfile_source = E'AGENT bash\nEXECUTABLE bash\n\nMODE pty\nMCP ON\n\n# Build logic: a single -c argument that runs an echo loop.\n# - Prints "ready" once on startup so tests can detect liveness via\n#   get_pod_snapshot.\n# - Reads stdin in a loop and echoes "got: <line>" so send_pod_input +\n#   get_pod_snapshot can be tested for round-trip.\n# Note: AGENT decl is the runner-side launch command (resolved through PATH),\n# not the agent slug. The slug "e2e-echo" lives only in the agents table.\narg "-c" "echo ready; while IFS= read -r line; do echo \\"got: $line\\"; done"\n'
WHERE slug = 'e2e-echo';

COMMIT;
