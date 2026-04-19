import { getExtensionService } from "@/lib/wasm-core";
export type {
  SkillRegistryAuthType, SkillRegistry, SkillRegistryOverride,
  SkillMarketItem, McpMarketItem, McpHeaderSchemaEntry, EnvVarSchemaEntry,
  InstalledSkill, InstalledMcpServer,
} from "./extensionTypes";

export const extensionApi = {
  listRepoSkills: async (repoId: number, scope?: string) => {
    const json = await getExtensionService().list_repo_skills(BigInt(repoId), scope ?? null);
    return JSON.parse(json);
  },
  listRepoMcpServers: async (repoId: number, scope?: string) => {
    const json = await getExtensionService().list_repo_mcp_servers(BigInt(repoId), scope ?? null);
    return JSON.parse(json);
  },
  updateSkill: async (repoId: number, installId: number, data: Record<string, unknown>) => {
    const json = await getExtensionService().update_skill(BigInt(repoId), BigInt(installId), JSON.stringify(data));
    return JSON.parse(json);
  },
  uninstallSkill: async (repoId: number, installId: number) => {
    await getExtensionService().uninstall_skill(BigInt(repoId), BigInt(installId));
  },
  updateMcpServer: async (repoId: number, installId: number, data: Record<string, unknown>) => {
    const json = await getExtensionService().update_mcp_server(BigInt(repoId), BigInt(installId), JSON.stringify(data));
    return JSON.parse(json);
  },
  uninstallMcpServer: async (repoId: number, installId: number) => {
    await getExtensionService().uninstall_mcp_server(BigInt(repoId), BigInt(installId));
  },
  listMarketSkills: async (query?: string, category?: string) => {
    const json = await getExtensionService().list_market_skills(query ?? null, category ?? null);
    return JSON.parse(json);
  },
  installSkillFromMarket: async (repoId: number, data: Record<string, unknown>) => {
    const json = await getExtensionService().install_skill_from_market(BigInt(repoId), JSON.stringify(data));
    return JSON.parse(json);
  },
  installSkillFromGitHub: async (repoId: number, data: Record<string, unknown>) => {
    const json = await getExtensionService().install_skill_from_github(BigInt(repoId), JSON.stringify(data));
    return JSON.parse(json);
  },
  installSkillFromUpload: async (repoId: number, file: File, scope?: string) => {
    const buf = new Uint8Array(await file.arrayBuffer());
    const json = await getExtensionService().install_skill_from_upload(BigInt(repoId), buf, file.name, scope ?? null);
    return JSON.parse(json);
  },
  listMarketMcpServers: async (query?: string, _category?: string, limit?: number, offset?: number) => {
    const json = await getExtensionService().list_market_mcp_servers(query ?? null, limit ?? null, offset ?? null);
    return JSON.parse(json);
  },
  installMcpFromMarket: async (repoId: number, data: Record<string, unknown>) => {
    const json = await getExtensionService().install_mcp_from_market(BigInt(repoId), JSON.stringify(data));
    return JSON.parse(json);
  },
  installCustomMcpServer: async (repoId: number, data: Record<string, unknown>) => {
    const json = await getExtensionService().install_custom_mcp_server(BigInt(repoId), JSON.stringify(data));
    return JSON.parse(json);
  },
  listSkillRegistries: async () => {
    const json = await getExtensionService().list_skill_registries();
    return JSON.parse(json);
  },
  listSkillRegistryOverrides: async () => {
    const json = await getExtensionService().list_skill_registry_overrides();
    return JSON.parse(json);
  },
  togglePlatformRegistry: async (registryId: number, disabled: boolean) => {
    const json = await getExtensionService().toggle_skill_registry(BigInt(registryId), JSON.stringify({ disabled }));
    return JSON.parse(json);
  },
  syncSkillRegistry: async (id: number) => {
    await getExtensionService().sync_skill_registry(BigInt(id));
  },
  deleteSkillRegistry: async (id: number) => {
    await getExtensionService().delete_skill_registry(BigInt(id));
  },
  createSkillRegistry: async (data: Record<string, unknown>) => {
    const json = await getExtensionService().create_skill_registry(JSON.stringify(data));
    return JSON.parse(json);
  },
};
