export interface CredentialProfileData {
  id: number;
  user_id: number;
  agent_slug: string;
  name: string;
  description?: string;
  is_runner_host: boolean;
  is_default: boolean;
  is_active: boolean;
  configured_fields?: string[];
  configured_values?: Record<string, string>;
  agent_name?: string;
  created_at: string;
  updated_at: string;
}

export interface CredentialProfilesByAgent {
  agent_slug: string;
  agent_name: string;
  profiles: CredentialProfileData[];
}

export interface CreateCredentialProfileRequest {
  name: string;
  description?: string;
  is_runner_host: boolean;
  credentials?: Record<string, string>;
  is_default?: boolean;
}

export interface UpdateCredentialProfileRequest {
  name?: string;
  description?: string;
  is_runner_host?: boolean;
  credentials?: Record<string, string>;
  is_default?: boolean;
  is_active?: boolean;
}

export interface RunnerHostInfo {
  available: boolean;
  description: string;
}

export function isRunnerHostProfile(profile: CredentialProfileData): boolean {
  return profile.is_runner_host;
}

export function getProfileStatusLabel(profile: CredentialProfileData): string {
  if (profile.is_runner_host) {
    return "RunnerHost";
  }
  if (profile.configured_fields && profile.configured_fields.length > 0) {
    return "Configured";
  }
  return "Not configured";
}
