export interface AgentMessage {
  id: number;
  sender_pod: string;
  receiver_pod: string;
  message_type: string;
  content: Record<string, unknown>;
  status: "pending" | "delivered" | "read" | "failed" | "dead_letter";
  correlation_id?: string;
  parent_message_id?: number;
  delivery_attempts: number;
  max_retries: number;
  delivered_at?: string;
  read_at?: string;
  created_at: string;
  updated_at: string;
}

export interface DeadLetterEntry {
  id: number;
  original_message_id: number;
  original_message?: AgentMessage;
  reason: string;
  final_attempt: number;
  moved_at: string;
  replayed_at?: string;
  replay_result?: string;
}
