export interface SSOConfig {
  domain: string;
  name: string;
  protocol: "oidc" | "saml" | "ldap";
  enforce_sso: boolean;
}

export interface SSODiscoverResponse {
  configs: SSOConfig[];
}

export interface LDAPAuthResponse {
  token: string;
  refresh_token: string;
  expires_at: string;
  token_type: string;
  user: {
    id: number;
    email: string;
    username: string;
    name?: string;
  };
}
