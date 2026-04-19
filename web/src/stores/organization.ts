import { create } from "zustand";
import { useMemo } from "react";
import { getOrgState, parseWasmAny } from "@/lib/wasm-core";

export interface OrganizationMember {
  id: number; user_id: number; username: string; email: string;
  name?: string; avatar_url?: string; role: "owner" | "admin" | "member"; joined_at: string;
}

export interface Organization {
  id: number; name: string; slug: string; logo_url?: string;
  subscription_plan: string; subscription_status: string; created_at: string; updated_at: string;
}

interface OrganizationState {
  _tick: number; isLoading: boolean; error: string | null;
  setOrganizations: (orgs: Organization[]) => void;
  setCurrentOrganization: (org: Organization | null) => void;
  addOrganization: (org: Organization) => void;
  updateOrganization: (id: number, updates: Partial<Organization>) => void;
  removeOrganization: (id: number) => void;
  setMembers: (members: OrganizationMember[]) => void;
  addMember: (member: OrganizationMember) => void;
  updateMember: (userId: number, updates: Partial<OrganizationMember>) => void;
  removeMember: (userId: number) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  reset: () => void;
}

const os = () => getOrgState();
const bump = () => useOrganizationStore.setState((s) => ({ _tick: s._tick + 1 }));

export function useOrganizations(): Organization[] {
  const tick = useOrganizationStore((s) => s._tick);
  return useMemo(() => JSON.parse(os().organizations_json()), [tick]);
}

export function useCurrentOrganization(): Organization | null {
  const tick = useOrganizationStore((s) => s._tick);
  return useMemo(() => parseWasmAny<Organization>(os().current_org_json()), [tick]);
}

export function useOrgMembers(): OrganizationMember[] {
  const tick = useOrganizationStore((s) => s._tick);
  return useMemo(() => JSON.parse(os().members_json()), [tick]);
}

export const useOrganizationStore = create<OrganizationState>((set) => ({
  _tick: 0, isLoading: false, error: null,

  setOrganizations: (orgs) => { os().set_organizations(JSON.stringify(orgs)); bump(); },
  setCurrentOrganization: (org) => { os().set_current_org(org ? JSON.stringify(org) : ""); bump(); },
  addOrganization: (org) => { os().add_organization(JSON.stringify(org)); bump(); },

  updateOrganization: (id, updates) => {
    const orgs: Organization[] = JSON.parse(os().organizations_json());
    const existing = orgs.find((o) => o.id === id);
    if (existing) os().update_organization(id, JSON.stringify({ ...existing, ...updates }));
    bump();
  },

  removeOrganization: (id) => { os().remove_organization(id); bump(); },
  setMembers: (members) => { os().set_members(JSON.stringify(members)); bump(); },
  addMember: (member) => { os().add_member(JSON.stringify(member)); bump(); },

  updateMember: (userId, updates) => {
    const members: OrganizationMember[] = JSON.parse(os().members_json());
    const existing = members.find((m) => m.user_id === userId);
    if (existing) os().update_member(userId, JSON.stringify({ ...existing, ...updates }));
    bump();
  },

  removeMember: (userId) => { os().remove_member(String(userId)); bump(); },
  setLoading: (isLoading) => set({ isLoading }),
  setError: (error) => set({ error }),

  reset: () => {
    os().set_organizations("[]"); os().set_current_org(""); os().set_members("[]");
    set({ _tick: 0, isLoading: false, error: null });
  },
}));
