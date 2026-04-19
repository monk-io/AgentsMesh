import { test, expect } from "../../fixtures/index";
import { clearAuthRateLimit } from "../../helpers/redis";

test.describe("User Credential Management API", () => {
  test.beforeEach(async () => { clearAuthRateLimit(); });

  test("list git credentials", async ({ api }) => {
    const res = await api.get("/api/v1/users/git-credentials");
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.credentials).toBeDefined();
  });

  test("get default git credential", async ({ api }) => {
    const res = await api.get("/api/v1/users/git-credentials/default");
    expect(res.status).toBe(200);
  });

  test("list repository providers", async ({ api }) => {
    const res = await api.get("/api/v1/users/repository-providers");
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.providers).toBeDefined();
  });

  test("list agent credentials", async ({ api }) => {
    const res = await api.get("/api/v1/users/agent-credentials");
    expect(res.status).toBe(200);
  });

  test("get user profile", async ({ api }) => {
    const res = await api.get("/api/v1/users/me");
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.user).toBeTruthy();
  });

  test("create and delete git credential", async ({ api }) => {
    const createRes = await api.post("/api/v1/users/git-credentials", {
      name: "E2E PAT Credential",
      credential_type: "pat",
      pat: "ghp_test123456789",
    });
    expect([200, 201]).toContain(createRes.status);
    const data = await createRes.json();
    const id = data.credential?.id || data.id;
    if (id) {
      const delRes = await api.delete(`/api/v1/users/git-credentials/${id}`);
      expect([200, 204]).toContain(delRes.status);
    }
  });

  test("test repository provider connection", async ({ api, db }) => {
    const id = db.queryValue("SELECT id FROM user_repository_providers LIMIT 1");
    if (!id) return;
    const res = await api.post(`/api/v1/users/repository-providers/${id}/test`);
    expect([200, 400, 500]).toContain(res.status);
  });
});
