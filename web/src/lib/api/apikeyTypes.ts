export interface APIKeyData {
  id: number;
  organization_id: number;
  name: string;
  description?: string;
  key_prefix: string;
  scopes: string[];
  is_enabled: boolean;
  expires_at?: string;
  last_used_at?: string;
  created_by: number;
  created_at: string;
  updated_at: string;
}

export interface CreateAPIKeyRequest {
  name: string;
  description?: string;
  scopes: string[];
  expires_in?: number;
}

export interface UpdateAPIKeyRequest {
  name?: string;
  description?: string;
  scopes?: string[];
  is_enabled?: boolean;
}
