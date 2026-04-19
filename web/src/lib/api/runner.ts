import { getRunnerService } from "@/lib/wasm-core";
export type { RunnerData, GRPCRegistrationToken, RunnerPodData, SandboxStatus, RelayConnectionInfo, RunnerLogData } from "./runnerTypes";

export interface RunnerAuthStatus {
  status: string;
  runner_id?: number;
  node_id?: string;
  organization_name?: string;
}

export const runnerApi = {
  list: async (status?: string) => {
    const json = await getRunnerService().fetch_runners(status ?? null);
    return JSON.parse(json);
  },
  get: async (id: number) => {
    const json = await getRunnerService().fetch_runner(BigInt(id));
    return JSON.parse(json);
  },
  fetchRunners: async (status?: string) => {
    const json = await getRunnerService().fetch_runners(status ?? null);
    return JSON.parse(json);
  },
  fetchRunner: async (id: number) => {
    const json = await getRunnerService().fetch_runner(BigInt(id));
    return JSON.parse(json);
  },
  fetchAvailableRunners: async () => {
    const json = await getRunnerService().fetch_available_runners();
    return JSON.parse(json);
  },
  update: async (id: number, data: Record<string, unknown>) => {
    const json = await getRunnerService().update_runner(BigInt(id), JSON.stringify(data));
    return JSON.parse(json);
  },
  delete: async (id: number) => {
    await getRunnerService().delete_runner(BigInt(id));
  },
  createToken: async (data?: Record<string, unknown>) => {
    const json = await getRunnerService().create_token(JSON.stringify(data ?? {}));
    return JSON.parse(json);
  },
  listLogs: async (id: number) => {
    const json = await getRunnerService().list_runner_logs(BigInt(id));
    return JSON.parse(json);
  },
  listPods: async (id: number, filters?: { status?: string; limit?: number; offset?: number }) => {
    const json = await getRunnerService().list_runner_pods(
      BigInt(id), filters?.status ?? null, filters?.limit ?? null, filters?.offset ?? null,
    );
    return JSON.parse(json);
  },
  querySandboxes: async (id: number, podKeys: string[]) => {
    const json = await getRunnerService().query_runner_sandboxes(BigInt(id), JSON.stringify({ pod_keys: podKeys }));
    return JSON.parse(json);
  },
  requestLogUpload: async (id: number) => {
    await getRunnerService().request_log_upload(BigInt(id));
  },
  upgrade: async (id: number, data?: Record<string, unknown>) => {
    const json = await getRunnerService().upgrade_runner(BigInt(id), JSON.stringify(data ?? {}));
    return JSON.parse(json);
  },
};

export const runnerAuthApi = {
  getAuthStatus: async (authKey: string): Promise<RunnerAuthStatus> => {
    const json = await getRunnerService().get_auth_status(authKey);
    return JSON.parse(json);
  },
  authorize: async (data: { auth_key: string }) => {
    await getRunnerService().authorize_runner(JSON.stringify(data));
  },
};
