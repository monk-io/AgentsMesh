import type { ApiFixture } from "../fixtures/api.fixture";
import { TEST_ORG_SLUG } from "./env";

// Thin wrappers over the AutopilotService control RPCs (ActionRequest /
// ApproveRequest in proto/autopilot/v1). Each takes the autopilot_controller
// key returned by createAutopilotForPod and issues the matching Connect call.

interface AutopilotClient {
  pauseAutopilotController(req: object): Promise<unknown>;
  resumeAutopilotController(req: object): Promise<unknown>;
  stopAutopilotController(req: object): Promise<unknown>;
  takeoverAutopilotController(req: object): Promise<unknown>;
  handbackAutopilotController(req: object): Promise<unknown>;
  approveAutopilotController(req: object): Promise<unknown>;
  getIterations(req: object): Promise<{ items?: AutopilotIterationItem[] }>;
}

export interface AutopilotIterationItem {
  iterationNumber?: number;
  status?: string;
  result?: string;
}

async function autopilot(api: ApiFixture): Promise<AutopilotClient> {
  const cc = await api.connect();
  return cc.autopilot as unknown as AutopilotClient;
}

export async function pauseAutopilot(api: ApiFixture, key: string): Promise<void> {
  await (await autopilot(api)).pauseAutopilotController({ orgSlug: TEST_ORG_SLUG, key });
}

export async function resumeAutopilot(api: ApiFixture, key: string): Promise<void> {
  await (await autopilot(api)).resumeAutopilotController({ orgSlug: TEST_ORG_SLUG, key });
}

export async function stopAutopilot(api: ApiFixture, key: string): Promise<void> {
  await (await autopilot(api)).stopAutopilotController({ orgSlug: TEST_ORG_SLUG, key });
}

export async function takeoverAutopilot(api: ApiFixture, key: string): Promise<void> {
  await (await autopilot(api)).takeoverAutopilotController({ orgSlug: TEST_ORG_SLUG, key });
}

export async function handbackAutopilot(api: ApiFixture, key: string): Promise<void> {
  await (await autopilot(api)).handbackAutopilotController({ orgSlug: TEST_ORG_SLUG, key });
}

// approveAutopilot answers a NEED_HUMAN_HELP pause. continueExecution=true
// resumes (granting additionalIterations), false stops the controller.
export async function approveAutopilot(
  api: ApiFixture,
  key: string,
  opts: { continueExecution: boolean; additionalIterations?: number },
): Promise<void> {
  await (await autopilot(api)).approveAutopilotController({
    orgSlug: TEST_ORG_SLUG,
    key,
    continueExecution: opts.continueExecution,
    additionalIterations: opts.additionalIterations ?? 0,
  });
}

// getIterationHistory reads the persisted iteration rows for a controller —
// the end-to-end read path for autopilot_iterations (migration 000158 column).
export async function getIterationHistory(api: ApiFixture, key: string): Promise<AutopilotIterationItem[]> {
  const resp = await (await autopilot(api)).getIterations({ orgSlug: TEST_ORG_SLUG, key });
  return resp.items ?? [];
}
