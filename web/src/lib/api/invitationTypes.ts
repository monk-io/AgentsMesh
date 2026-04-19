export interface Invitation {
  id: number;
  organization_id: number;
  email: string;
  role: "admin" | "member";
  expires_at: string;
  accepted_at?: string;
  created_at: string;
}

export interface InvitationInfo {
  id: number;
  email: string;
  role: string;
  organization_id: number;
  organization_name: string;
  organization_slug: string;
  inviter_name: string;
  expires_at: string;
  is_expired: boolean;
}

export interface PendingInvitation {
  id: number;
  organization_id: number;
  organization_name: string;
  organization_slug: string;
  role: string;
  expires_at: string;
  token: string;
}
