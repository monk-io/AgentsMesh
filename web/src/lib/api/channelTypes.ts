export interface MentionPayload {
  type: "user" | "pod";
  id: string;
}

export interface ChannelData {
  id: number;
  organization_id?: number;
  name: string;
  description?: string;
  document?: string;
  repository_id?: number;
  ticket_id?: number;
  ticket_slug?: string;
  created_by_pod?: string;
  created_by_user_id?: number;
  is_archived: boolean;
  created_at?: string;
  updated_at?: string;
}

export interface ChannelMessage {
  id: number;
  channel_id: number;
  sender_pod?: string;
  sender_user_id?: number;
  message_type?: "text" | "system" | "code" | "command";
  content: string;
  metadata?: Record<string, unknown>;
  edited_at?: string;
  is_deleted?: boolean;
  created_at?: string;
  sender_pod_info?: {
    pod_key: string;
    alias?: string;
    agent?: {
      name: string;
    };
  };
  sender_user?: {
    id: number;
    username: string;
    name?: string;
    avatar_url?: string;
  };
}
