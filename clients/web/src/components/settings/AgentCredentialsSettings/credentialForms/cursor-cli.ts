import type { CredentialFormSpec } from "./types";

// AgentFile source: backend/migrations/000157_add_cursor_cli_agent.up.sql
//   ENV CURSOR_API_KEY SECRET OPTIONAL
// cursor-agent defaults to `cursor-agent login` OAuth (token inherited from
// HOME); CURSOR_API_KEY is the headless/key-based alternative. Custom ENV
// stays open for proxy/model overrides.
export const cursorCliFormSpec: CredentialFormSpec = {
  agentSlug: "cursor-cli",
  fields: [
    {
      kind: "secret",
      envKey: "CURSOR_API_KEY",
      label: "settings.credentialForm.cursor.apiKey",
      description: "settings.credentialForm.cursor.apiKeyHint",
    },
  ],
  allowCustomEnv: true,
  customEnvHint: "settings.credentialForm.cursor.customEnvHint",
};
