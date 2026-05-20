import { test, expect } from "../../fixtures/index";
import { TEST_ORG_SLUG } from "../../helpers/env";
import { clearAuthRateLimit } from "../../helpers/redis";

const ORG_BASE = `/api/v1/orgs/${TEST_ORG_SLUG}`;

test.describe("Backend wrapper envelope contracts", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("ticket list keeps total/limit/offset", async ({ api }) => {
    const res = await api.get(`${ORG_BASE}/tickets?limit=5&offset=0`);
    expect(res.status).toBe(200);
    const body = await res.json();
    expect(body).toHaveProperty("tickets");
    expect(body).toHaveProperty("total");
    expect(body).toHaveProperty("limit");
    expect(body).toHaveProperty("offset");
    expect(body.limit).toBe(5);
    expect(body.offset).toBe(0);
  });

  // NOTE: the DLQ envelope test used to live here but messaging service is
  // not wired into cmd/server (no MessageService instance, no migration for
  // agent_messages / agent_message_dead_letters). When the feature is wired
  // up, restore: GET messages/dlq?limit=10&offset=0 → { entries, total }.

  test("pod create response carries pod envelope", async ({ api }) => {
    const runners = (await (await api.get(`${ORG_BASE}/runners/available`)).json()).runners;
    if (!runners?.length) { test.skip(); return; }
    const agents = (await (await api.get(`${ORG_BASE}/agents`)).json()).builtin_agents;
    if (!agents?.length) { test.skip(); return; }

    const res = await api.post(`${ORG_BASE}/pods`, {
      runner_id: runners[0].id,
      agent_slug: agents[0].slug,
    });
    expect([200, 201]).toContain(res.status);
    const body = await res.json();
    expect(body).toHaveProperty("pod");
    expect(body.pod?.pod_key || body.pod?.key).toBeTruthy();
    if (body.pod?.pod_key || body.pod?.key) {
      const key = body.pod.pod_key || body.pod.key;
      await api.post(`${ORG_BASE}/pods/${key}/terminate`, {});
    }
  });

  test("loop runs list keeps pagination shape", async ({ api, db }) => {
    // Self-seed a loop so the test doesn't depend on whatever residue
    // previous tests happened to leave behind. The wrapper-envelope check
    // is about wire shape (runs / total / limit / offset), not behaviour —
    // an empty runs array is the expected payload for a freshly-created loop.
    const loopName = `E2E Envelope Loop ${Date.now()}`;
    db.cleanup(`DELETE FROM loops WHERE name LIKE 'E2E Envelope Loop%'`);
    const createRes = await api.post(`${ORG_BASE}/loops`, {
      name: loopName,
      agent_slug: "claude-code",
      prompt_template: "noop",
    });
    expect([200, 201]).toContain(createRes.status);
    const loop = (await createRes.json()).loop;
    expect(loop?.slug).toBeTruthy();

    try {
      const res = await api.get(`${ORG_BASE}/loops/${loop.slug}/runs?limit=10&offset=0`);
      expect(res.status).toBe(200);
      const body = await res.json();
      expect(body).toHaveProperty("runs");
      expect(body).toHaveProperty("total");
      expect(body).toHaveProperty("limit");
      expect(body).toHaveProperty("offset");
    } finally {
      await api.delete(`${ORG_BASE}/loops/${loop.slug}`);
      db.cleanup(`DELETE FROM loops WHERE name LIKE 'E2E Envelope Loop%'`);
    }
  });
});
