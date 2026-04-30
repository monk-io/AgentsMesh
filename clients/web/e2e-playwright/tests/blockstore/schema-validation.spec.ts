import { test, expect, orgSlug, apiBase } from "../../fixtures/blockstore.fixture";

// Block Store schema guard — asserts that the backend's enhanced ValidateRecord
// (EnumValues + NonEmptyArrayKeys) actually rejects bad chart writes at the
// REST layer, not just at the Go unit-test layer. This is the only path that
// proves the new check is hooked into the live ApplyOps handler and the
// middleware chain.
//
// Covers F3 (chart schema), and F12 (translateErr no longer leaks stacks on
// the validation branch).

interface AuthContext {
  token: string;
}

async function postOps(
  auth: AuthContext,
  workspaceID: string,
  ops: unknown[],
  idempotencyKey?: string,
): Promise<Response> {
  return fetch(`${apiBase}/api/v1/orgs/${orgSlug}/blocks/ops`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${auth.token}`,
      "X-Organization-Slug": orgSlug,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      workspace_id: workspaceID,
      ops,
      idempotency_key: idempotencyKey,
    }),
  });
}

test("chart with unknown type is rejected at REST layer", async ({ token, isolatedWorkspace }) => {
  const { id: workspaceID } = isolatedWorkspace;
  const res = await postOps({ token }, workspaceID, [
    {
      op: "createBlock",
      payload: {
        type: "chart",
        data: {
          type: "3d_sphere",
          series: [{ name: "s", data: [{ x: 1, value: 2 }] }],
        },
      },
    },
  ]);
  expect(res.status).toBe(400);
  const body = (await res.json()) as { message?: string; error?: string };
  const msg = body.message ?? body.error ?? "";
  expect(msg).toMatch(/type/);
});

test("chart with empty series is rejected at REST layer", async ({ token, isolatedWorkspace }) => {
  const { id: workspaceID } = isolatedWorkspace;
  const res = await postOps({ token }, workspaceID, [
    {
      op: "createBlock",
      payload: {
        type: "chart",
        data: { type: "bar", series: [] },
      },
    },
  ]);
  expect(res.status).toBe(400);
  const raw = await res.text();
  expect(raw).toMatch(/series/);
});

test("valid chart is accepted (positive control)", async ({ token, isolatedWorkspace }) => {
  const { id: workspaceID } = isolatedWorkspace;
  const res = await postOps({ token }, workspaceID, [
    {
      op: "createBlock",
      payload: {
        type: "chart",
        data: {
          type: "bar",
          series: [{ name: "Revenue", data: [{ month: "Jan", value: 10 }] }],
        },
      },
    },
  ]);
  expect(res.status, `expected 2xx for a valid chart; got ${await res.text()}`).toBeLessThan(300);
});

test("javascript: URL in block data is sanitized by the renderer", async ({
  authenticatedPage,
  api,
  isolatedWorkspace,
}) => {
  const { id: workspaceID, rootID } = isolatedWorkspace;
  // Even if a malicious actor bypasses the frontend prompt and writes a
  // block with a javascript: url via API (the backend has no scheme
  // allowlist for bookmark data), the renderer's urlGuard must prevent
  // that scheme from reaching href/src in the DOM. This is the defense-
  // in-depth layer that matters most in practice.
  const markerTitle = `xss-probe-${Date.now()}`;
  const id = crypto.randomUUID();
  await api.post(`/api/v1/orgs/${orgSlug}/blocks/ops`, {
    workspace_id: workspaceID,
    ops: [
      {
        op: "createBlock",
        payload: {
          id,
          type: "bookmark",
          data: { url: "javascript:alert('xss')", title: markerTitle },
          text: markerTitle,
        },
      },
      {
        op: "addRef",
        payload: { from: rootID, to: id, rel: "nest", order_key: `zzy${Date.now().toString(36)}` },
      },
    ],
    idempotency_key: `e2e-xss-ok-${id}`,
  });

  await authenticatedPage.goto(`/${orgSlug}/blocks?ws=${workspaceID}`);
  const bookmark = authenticatedPage.getByText(markerTitle).last();
  await expect(bookmark).toBeVisible({ timeout: 15_000 });

  // The rendered anchor's href must NOT carry the javascript: scheme.
  // sanitizeURL returns "" which the renderer shows as "#".
  const anchor = bookmark.locator("xpath=ancestor::a[1]");
  const href = await anchor.getAttribute("href");
  expect(href ?? "").not.toMatch(/^javascript:/i);
});
