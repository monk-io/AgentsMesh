export interface OrganizationMember {
  id: number;
  user_id: number;
  role: "owner" | "admin" | "member";
  joined_at: string;
  user?: {
    id: number;
    email: string;
    username: string;
    name?: string;
    avatar_url?: string;
  };
}

export interface OrganizationData {
  id: number;
  name: string;
  slug: string;
  role?: string;
  logo_url?: string;
  subscription_plan?: string;
  subscription_status?: string;
  created_at?: string;
  updated_at?: string;
}
