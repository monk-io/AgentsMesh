import { randomUUID } from "crypto";

import { test, expect, orgSlug, apiBase } from "../../fixtures/blockstore.fixture";

// Covers F2 (SSRF guard) and F6 (iframe sandbox). These are the two layers
// that together stop a malicious Agent from (a) using the backend as an
// internal-network HTTP probe via trigger webhooks and (b) escaping the
// embed iframe to manipulate the host page.
//
// After the MCP-bridge refactor, the SSRF guard lives in the blockstore
// service layer (validateTriggerDefData) so we hit /blocks/ops directly
// with a trigger_def createBlock. Previously this went through a
// /blocks/mcp/call shim that has since been deleted.

async function postOps(
  token: string,
  workspaceID: string,
  triggerName: string,
  action: Record<string, unknown>,
): Promise<Response> {
  return fetch(`${apiBase}/api/v1/orgs/${orgSlug}/blocks/ops`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${token}`,
      "X-Organization-Slug": orgSlug,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      workspace_id: workspaceID,
      ops: [
        {
          op: "createBlock",
          payload: {
            type: "trigger_def",
            data: {
              name: triggerName,
              target_type: "task",
              on: "create",
              action,
              enabled: true,
            },
            text: triggerName,
          },
        },
      ],
      idempotency_key: `e2e-ssrf-${triggerName}`,
    }),
  });
}

test("trigger.define with loopback URL is rejected", async ({ token, isolatedWorkspace }) => {
  const { id: workspaceID } = isolatedWorkspace;
  const res = await postOps(token, workspaceID, `ssrf-loopback-${Date.now()}`, {
    kind: "webhook",
    url: "http://127.0.0.1:5432/hook",
  });
  expect(res.status).toBe(400);
  const body = await res.text();
  expect(body).toMatch(/action\.url|private|reserved/i);
});

test("trigger.define with AWS metadata URL is rejected", async ({ token, isolatedWorkspace }) => {
  const { id: workspaceID } = isolatedWorkspace;
  const res = await postOps(token, workspaceID, `ssrf-aws-${Date.now()}`, {
    kind: "webhook",
    url: "http://169.254.169.254/latest/meta-data/",
  });
  expect(res.status).toBe(400);
});

test("trigger.define with RFC1918 URL is rejected", async ({ token, isolatedWorkspace }) => {
  const { id: workspaceID } = isolatedWorkspace;
  const res = await postOps(token, workspaceID, `ssrf-rfc1918-${Date.now()}`, {
    kind: "webhook",
    url: "http://10.0.0.1/hook",
  });
  expect(res.status).toBe(400);
});

test("trigger.define with javascript: scheme is rejected", async ({ token, isolatedWorkspace }) => {
  const { id: workspaceID } = isolatedWorkspace;
  const res = await postOps(token, workspaceID, `ssrf-js-${Date.now()}`, {
    kind: "webhook",
    url: "javascript:alert(1)",
  });
  expect(res.status).toBe(400);
});

test("embed iframe carries a strict sandbox attribute", async ({
  authenticatedPage,
  api,
  isolatedWorkspace,
}) => {
  const { id: workspaceID, rootID } = isolatedWorkspace;
  // Seed an embed block pointing at a known YouTube URL so the renderer
  // picks the iframe branch. We use raw API write so the test doesn't
  // depend on window.prompt which Playwright handles awkwardly.
  const id = randomUUID();
  await api.post(`/api/v1/orgs/${orgSlug}/blocks/ops`, {
    workspace_id: workspaceID,
    ops: [
      {
        op: "createBlock",
        payload: {
          id,
          type: "embed",
          data: { url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ", provider: "youtube" },
          text: "yt",
        },
      },
      {
        op: "addRef",
        payload: { from: rootID, to: id, rel: "nest", order_key: `zzx${Date.now().toString(36)}` },
      },
    ],
    idempotency_key: `e2e-sandbox-${id}`,
  });

  await authenticatedPage.goto(`/${orgSlug}/blocks?ws=${workspaceID}`);
  const iframe = authenticatedPage.locator(`iframe[src*="youtube.com/embed"]`).last();
  await expect(iframe).toBeAttached({ timeout: 15_000 });

  const sandbox = await iframe.getAttribute("sandbox");
  expect(sandbox, "iframe must have sandbox attribute").toBeTruthy();
  // The allowed tokens — no allow-top-navigation, no allow-forms.
  expect(sandbox).toContain("allow-scripts");
  expect(sandbox).toContain("allow-same-origin");
  expect(sandbox).not.toContain("allow-top-navigation");
  expect(sandbox).not.toContain("allow-forms");
});
