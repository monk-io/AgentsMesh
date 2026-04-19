import { create } from "zustand";
import { getUserState } from "@/lib/wasm-core";

export interface UserIdentity {
  id: number;
  provider: "github" | "google" | "gitlab" | "gitee";
  provider_user_id: string;
  provider_username?: string;
  created_at: string;
}

export interface User {
  id: number;
  email: string;
  username: string;
  name?: string;
  avatar_url?: string;
  is_active: boolean;
  last_login_at?: string;
  created_at: string;
  updated_at: string;
}

export interface UserProfile extends User {
  identities: UserIdentity[];
  organizations: Array<{
    id: number;
    name: string;
    slug: string;
    role: string;
  }>;
}

function readProfile(): UserProfile | null {
  const val = getUserState().profile_json();
  if (!val) return null;
  return typeof val === "string" ? JSON.parse(val) : val;
}

interface UserState {
  profile: UserProfile | null;
  isLoading: boolean;
  error: string | null;
  setProfile: (profile: UserProfile | null) => void;
  updateProfile: (updates: Partial<UserProfile>) => void;
  addIdentity: (identity: UserIdentity) => void;
  removeIdentity: (provider: string) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  reset: () => void;
}

export const useUserStore = create<UserState>((set) => ({
  profile: null, isLoading: false, error: null,

  setProfile: (profile) => {
    getUserState().set_profile(profile ? JSON.stringify(profile) : "");
    set({ profile: readProfile() });
  },

  updateProfile: (updates) => {
    const cur = readProfile();
    if (!cur) return;
    getUserState().set_profile(JSON.stringify({ ...cur, ...updates }));
    set({ profile: readProfile() });
  },

  addIdentity: (identity) => {
    getUserState().add_identity(JSON.stringify(identity));
    set({ profile: readProfile() });
  },

  removeIdentity: (provider) => {
    const cur = readProfile();
    if (!cur) return;
    getUserState().set_profile(JSON.stringify({
      ...cur,
      identities: cur.identities.filter((i) => i.provider !== provider),
    }));
    set({ profile: readProfile() });
  },

  setLoading: (isLoading) => set({ isLoading }),
  setError: (error) => set({ error }),

  reset: () => {
    getUserState().set_profile("");
    set({ profile: null, isLoading: false, error: null });
  },
}));
