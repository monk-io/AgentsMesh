export type AIProviderType = "claude" | "openai" | "gemini" | "codex";

export interface UserAgentPodSettings {
  id: number;
  user_id: number;
  preparation_script?: string;
  preparation_timeout: number;
  default_agent_slug?: string;
  default_model?: string;
  default_perm_mode?: string;
  terminal_font_size?: number;
  terminal_font_family?: string;
}

export interface UserAIProvider {
  id: number;
  user_id: number;
  provider_type: AIProviderType;
  name: string;
  is_default: boolean;
  is_enabled: boolean;
  created_at: string;
  updated_at: string;
}

export interface UpdateSettingsRequest {
  preparation_script?: string;
  preparation_timeout?: number;
  default_model?: string;
  default_perm_mode?: string;
  terminal_font_size?: number;
  terminal_font_family?: string;
}

export interface CreateProviderRequest {
  provider_type: AIProviderType;
  name: string;
  credentials: Record<string, string>;
  is_default?: boolean;
}

export interface UpdateProviderRequest {
  name?: string;
  credentials?: Record<string, string>;
  is_enabled?: boolean;
  is_default?: boolean;
}
