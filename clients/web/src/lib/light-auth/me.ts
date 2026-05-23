// /api/v1/users/me equivalent over Connect (proto.user.v1.UserService/GetMe).
// Three (auth) pages (invite acceptance, onboarding, runners/authorize) read
// the viewer's email just to display "Signed in as <email>" / derive a
// default workspace name without booting wasm. Returns null on any failure —
// these pages use it as best-effort UI sugar and must keep working when the
// call fails.

import { lightConnect } from "./api-fetch";

export interface LightUser {
  id: number;
  email: string;
  username: string;
  name?: string;
  avatar_url?: string;
}

interface ConnectGetMeResponse {
  user?: {
    id: number | string;
    email: string;
    username: string;
    name?: string;
    avatarUrl?: string;
  };
}

export async function lightFetchMe(): Promise<LightUser | null> {
  try {
    const resp = await lightConnect<Record<string, never>, ConnectGetMeResponse>(
      "proto.user.v1.UserService",
      "GetMe",
      {},
      { authenticated: true },
    );
    const u = resp?.user;
    if (!u) return null;
    return {
      id: Number(u.id),
      email: u.email,
      username: u.username,
      name: u.name,
      avatar_url: u.avatarUrl,
    };
  } catch {
    return null;
  }
}
