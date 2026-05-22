// Migrated R5+: Connect-RPC only (no REST middle layer).
import { test, expect } from "../../fixtures/index";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("Agent Credentials API", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  /**
   * TC-AGENTCRED-001: List agent credentials
   */
  test("list agent credentials", async ({ api }) => {
    const cc = await api.connect();
    const res = await cc.userAgentCredential.listAgentCredentialProfiles({}) as { items?: unknown[] };
    expect(res).toBeTruthy();
  });

  test("list agent credentials without auth returns unauthenticated", async ({ api }) => {
    const cc = api.connectWithToken("bad");
    await expect(
      cc.userAgentCredential.listAgentCredentialProfiles({})
    ).rejects.toMatchObject({ status: 401 });
  });

  /**
   * TC-AGENTCRED-002: Create agent credential profile
   */
  test("create agent credential profile", async ({ api, db }) => {
    db.cleanup(
      `DELETE FROM user_agent_credential_profiles WHERE name = 'E2E Agent Cred Test'`
    );
    const cc = await api.connect();
    const created = await cc.userAgentCredential.createAgentCredentialProfile({
      agentSlug: "claude-code",
      name: "E2E Agent Cred Test",
      description: "E2E test credential",
      isRunnerHost: false,
      credentials: { ANTHROPIC_API_KEY: "sk-ant-test-e2e" },
    }) as { id: string | number };
    expect(created.id).toBeTruthy();

    db.cleanup(
      `DELETE FROM user_agent_credential_profiles WHERE name = 'E2E Agent Cred Test'`
    );
  });

  /**
   * TC-AGENTCRED-004: Delete agent credential
   */
  test("delete non-existent agent credential returns not_found", async ({ api }) => {
    const cc = await api.connect();
    await expect(
      cc.userAgentCredential.deleteAgentCredentialProfile({ id: 999999 })
    ).rejects.toMatchObject({ status: 404 });
  });
});
