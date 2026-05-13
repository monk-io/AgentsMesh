import { getExtensionService } from "@/lib/wasm-core";
export type {
  SkillRegistryAuthType, SkillRegistry, SkillRegistryOverride,
  SkillMarketItem, McpMarketItem, McpHeaderSchemaEntry, EnvVarSchemaEntry,
  InstalledSkill, InstalledMcpServer,
} from "./extensionTypes";

// Multipart skill upload stays REST forever — Connect-RPC doesn't handle
// multipart/form-data. Everything else in this domain went to Connect.
export const extensionApi = {
  installSkillFromUpload: async (repoId: number, file: File, scope?: string) => {
    const buf = new Uint8Array(await file.arrayBuffer());
    const json = await getExtensionService().install_skill_from_upload(BigInt(repoId), buf, file.name, scope ?? null);
    return JSON.parse(json);
  },
};
