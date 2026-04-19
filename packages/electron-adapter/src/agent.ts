import { invoke } from "./invoke";
import type { IAgentService } from "@agentsmesh/service-interface";

export class ElectronAgentService implements IAgentService {
  async list_agents(): Promise<string> {
    return invoke<string>("agentListAgents");
  }

  async list_providers(): Promise<string> {
    return invoke<string>("agentListProviders");
  }

  async create_provider(json: string): Promise<string> {
    return invoke<string>("agentCreateProvider", json);
  }

  async update_provider(id: bigint, json: string): Promise<string> {
    return invoke<string>("agentUpdateProvider", Number(id), json);
  }

  async delete_provider(id: bigint): Promise<void> {
    await invoke<void>("agentDeleteProvider", Number(id));
  }

  async set_default_provider(id: bigint): Promise<void> {
    await invoke<void>("agentSetDefaultProvider", Number(id));
  }

  async get_config_schema(agentSlug: string): Promise<string> {
    return invoke<string>("agentGetConfigSchema", agentSlug);
  }

  async get_user_config(agentSlug: string): Promise<string> {
    return invoke<string>("agentGetUserConfig", agentSlug);
  }

  async set_user_config(agentSlug: string, json: string): Promise<string> {
    return invoke<string>("agentSetUserConfig", agentSlug, json);
  }

  async delete_user_config(agentSlug: string): Promise<void> {
    await invoke<void>("agentDeleteUserConfig", agentSlug);
  }

  async list_user_configs(): Promise<string> {
    return invoke<string>("agentListUserConfigs");
  }

  async get_agentpod_settings(): Promise<string> {
    return invoke<string>("agentGetAgentpodSettings");
  }

  async update_agentpod_settings(json: string): Promise<string> {
    return invoke<string>("agentUpdateAgentpodSettings", json);
  }
}
