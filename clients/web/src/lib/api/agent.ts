import { getAgentService } from "@/lib/wasm-core";
export type { AgentData, ConfigField, ConfigSchema, CredentialField, UserAgentConfigData } from "./agentTypes";

export const agentApi = {
  list: async () => {
    const json = await getAgentService().list_agents();
    return JSON.parse(json);
  },
  getConfigSchema: async (agentSlug: string) => {
    const json = await getAgentService().get_config_schema(agentSlug);
    return JSON.parse(json);
  },
};
