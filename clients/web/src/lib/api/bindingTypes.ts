export interface PodBinding {
  id: number;
  organization_id: number;
  initiator_pod: string;
  target_pod: string;
  granted_scopes: string[];
  pending_scopes: string[];
  status: "pending" | "active" | "rejected" | "inactive" | "expired";
  requested_at?: string;
  responded_at?: string;
  expires_at?: string;
  rejection_reason?: string;
  created_at: string;
  updated_at: string;
}
