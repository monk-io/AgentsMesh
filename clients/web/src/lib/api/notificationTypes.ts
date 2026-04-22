export interface NotificationPreference {
  source: string;
  entity_id?: string;
  is_muted: boolean;
  channels: Record<string, boolean>;
}
