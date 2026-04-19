import { invoke } from "./invoke";
import type { IAuthManager } from "@agentsmesh/service-interface";

export class ElectronAuthService implements IAuthManager {
  private _token: string | undefined;
  private _refreshToken: string | undefined;
  private _organizationsCache = "[]";
  private _currentUserCache: string | null = null;
  private _currentOrgCache: string | null = null;

  readonly base_url: string;

  constructor(baseUrl: string) {
    this.base_url = baseUrl;
  }

  get_token(): string | undefined { return this._token; }
  get_refresh_token(): string | undefined { return this._refreshToken; }
  is_authenticated(): boolean { return !!this._token; }

  get_current_user_json(): unknown { return this._currentUserCache; }
  get_current_org_json(): unknown { return this._currentOrgCache; }
  get_organizations_json(): string { return this._organizationsCache; }

  switch_org(slug: string): void {
    invoke("authSwitchOrg", slug).catch(() => {});
    const orgs = JSON.parse(this._organizationsCache) as { slug: string }[];
    const org = orgs.find(o => o.slug === slug);
    if (org) this._currentOrgCache = JSON.stringify(org);
  }

  restore_session(): Promise<boolean> {
    return this.restoreSessionAsync();
  }

  private async restoreSessionAsync(): Promise<boolean> {
    try {
      const restored = await invoke<boolean>("authRestoreSession");
      if (!restored) return false;
      this._token = await invoke<string | undefined>("authGetToken");
      this._currentUserCache = (await invoke<string | null>("authGetCurrentUserJson")) ?? null;
      this._currentOrgCache = (await invoke<string | null>("authGetCurrentOrgJson")) ?? null;
      return !!this._token;
    } catch {
      return false;
    }
  }

  async login(email: string, password: string): Promise<string> {
    const result = await invoke<string>("authLogin", email, password);
    const parsed = JSON.parse(result) as { token?: string; refresh_token?: string; user?: unknown };
    this._token = parsed.token;
    this._refreshToken = parsed.refresh_token;
    if (parsed.user) this._currentUserCache = JSON.stringify(parsed.user);
    return result;
  }

  async logout(): Promise<void> {
    await invoke<void>("authLogout");
    this._token = undefined;
    this._refreshToken = undefined;
    this._currentUserCache = null;
    this._currentOrgCache = null;
  }

  async refresh_token(): Promise<string> {
    const result = await invoke<string>("authRefreshToken");
    const parsed = JSON.parse(result) as { token?: string; refresh_token?: string };
    this._token = parsed.token;
    this._refreshToken = parsed.refresh_token;
    return result;
  }

  async fetch_organizations(): Promise<string> {
    const result = await invoke<string>("authFetchOrganizations");
    this._organizationsCache = result;
    // also refresh current org
    this._currentOrgCache = (await invoke<string | null>("authGetCurrentOrgJson")) ?? null;
    return result;
  }

  static new_with_storage(baseUrl: string, _storage: unknown): IAuthManager {
    return new ElectronAuthService(baseUrl);
  }
}
