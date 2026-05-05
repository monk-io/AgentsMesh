import { getPodService } from "@/lib/wasm-core";

export const podApi = {
  create: async (data: Record<string, unknown>) => {
    const json = await getPodService().create_pod(JSON.stringify(data));
    return JSON.parse(json);
  },
  terminate: async (podKey: string) => {
    await getPodService().terminate_pod(podKey);
  },
};
