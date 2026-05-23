// Migrated R5+: Connect-RPC only (no REST middle layer).
import { test, expect } from "../../fixtures/index";
import { clearAuthRateLimit } from "../../helpers/redis";
import { CLEANUP } from "../../helpers/test-data";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { makeConnectClient } from "../../helpers/connect-client";

/**
 * CRUD supplement tests — filling gaps in existing modules.
 */
test.describe("CRUD Supplements", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("update agent credential profile name", async ({ api, db }) => {
    test.skip(true, "UserAgentCredentialService removed in PR #404; superseded by EnvBundle");
    // user_agent_credential_profiles has UNIQUE(user_id, agent_slug, name);
    // a residue from a prior failed run would make POST return
    // ALREADY_EXISTS and silently skip. Pre-clean.
    db.cleanup(
      `DELETE FROM user_agent_credential_profiles WHERE name IN ('E2E Update Cred', 'E2E Updated Cred')`
    );
    const cc = await api.connect();
    let created: { id?: number };
    try {
      created = await cc.userAgentCredential.createAgentCredentialProfile({
        agentSlug: "claude-code",
        name: "E2E Update Cred",
        credentials: { ANTHROPIC_API_KEY: "sk-test" },
        isRunnerHost: false,
        isDefault: false,
      }) as { id?: number };
    } catch (err) {
      if ((err as { status: number }).status === 404) { test.skip(); return; }
      throw err;
    }
    if (!created.id) { test.skip(); return; }

    await cc.userAgentCredential.updateAgentCredentialProfile({
      id: Number(created.id),
      name: "E2E Updated Cred",
    });

    db.cleanup(
      `DELETE FROM user_agent_credential_profiles WHERE name IN ('E2E Update Cred', 'E2E Updated Cred')`
    );
  });

  test("set agent credential as default", async ({ api, db }) => {
    test.skip(true, "UserAgentCredentialService removed in PR #404; superseded by EnvBundle");
    db.cleanup(
      `DELETE FROM user_agent_credential_profiles WHERE name = 'E2E Default Cred'`
    );
    const cc = await api.connect();
    let created: { id?: number };
    try {
      created = await cc.userAgentCredential.createAgentCredentialProfile({
        agentSlug: "claude-code",
        name: "E2E Default Cred",
        credentials: { ANTHROPIC_API_KEY: "sk-test" },
        isRunnerHost: false,
        isDefault: false,
      }) as { id?: number };
    } catch (err) {
      if ((err as { status: number }).status === 404) { test.skip(); return; }
      throw err;
    }
    if (!created.id) { test.skip(); return; }

    await cc.userAgentCredential.setDefaultAgentCredentialProfile({
      id: Number(created.id),
    });

    db.cleanup(
      `DELETE FROM user_agent_credential_profiles WHERE name = 'E2E Default Cred'`
    );
  });

  test("change organization member role", async ({ api, db }) => {
    const email = "role-change-e2e@test.local";
    try { db.cleanup(CLEANUP.userByEmail(email)); } catch { /* */ }

    // Unauthenticated register call — no token needed; cc.auth.register is
    // public per AuthService proto annotations.
    const anonCc = makeConnectClient(null);
    await anonCc.auth.register({
      email, username: "rolechangee2e", password: "TestPass123!", name: "Role Change",
    });

    const orgId = db.queryValue(
      `SELECT id FROM organizations WHERE slug = '${TEST_ORG_SLUG}'`
    );
    const userId = db.queryValue(`SELECT id FROM users WHERE email = '${email}'`);
    if (!orgId || !userId) { test.skip(); return; }

    db.setup(
      `INSERT INTO organization_members (organization_id, user_id, role) VALUES (${orgId}, ${userId}, 'member') ON CONFLICT DO NOTHING`
    );

    const cc = await api.connect();
    await cc.org.updateMemberRole({
      orgSlug: TEST_ORG_SLUG,
      userId: Number(userId),
      role: "admin",
    });

    db.cleanup(CLEANUP.userByEmail(email));
  });

  test("set repository provider as default", async ({ api }) => {
    const cc = await api.connect();
    const created = await cc.userRepositoryProvider.createRepositoryProvider({
      providerType: "github",
      name: "E2E Default Provider",
      baseUrl: "https://api.github.com",
      botToken: "ghp_default_test",
    }) as { id?: number };
    if (!created.id) { test.skip(); return; }

    await cc.userRepositoryProvider.setDefaultRepositoryProvider({
      id: Number(created.id),
    });

    await cc.userRepositoryProvider.deleteRepositoryProvider({
      id: Number(created.id),
    });
  });

  test("create pod with repository association", async ({ api }) => {
    const cc = await api.connect();
    const runners = await cc.runner.listAvailableRunners({ orgSlug: TEST_ORG_SLUG }) as {
      items: Array<{ id: number }>;
    };
    if (!runners.items?.length) { test.skip(); return; }

    const agents = await cc.agent.listAgents({ orgSlug: TEST_ORG_SLUG }) as {
      builtinAgents: Array<{ slug: string }>;
    };
    if (!agents.builtinAgents?.length) { test.skip(); return; }

    const repos = await cc.repository.listRepositories({ orgSlug: TEST_ORG_SLUG }) as {
      items: Array<{ id: number }>;
    };

    const body: Record<string, unknown> = {
      orgSlug: TEST_ORG_SLUG,
      runnerId: Number(runners.items[0].id),
      agentSlug: agents.builtinAgents[0].slug,
      cols: 80,
      rows: 24,
    };
    if (repos.items?.length) {
      body.repositoryId = Number(repos.items[0].id);
    }

    const res = await cc.pod.createPod(body) as { pod?: { podKey?: string } };
    const podKey = res.pod?.podKey;
    if (podKey) {
      await cc.pod.terminatePod({ orgSlug: TEST_ORG_SLUG, podKey });
    }
  });
});
