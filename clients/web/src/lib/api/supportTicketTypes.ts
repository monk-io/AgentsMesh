export interface SupportTicket {
  id: number;
  user_id: number;
  title: string;
  category: string;
  status: string;
  priority: string;
  assigned_admin_id?: number;
  created_at: string;
  updated_at: string;
  resolved_at?: string;
}

export interface SupportTicketMessage {
  id: number;
  ticket_id: number;
  user_id: number;
  content: string;
  is_admin_reply: boolean;
  created_at: string;
  user?: { id: number; name: string; email: string; avatar_url?: string };
  attachments?: SupportTicketAttachment[];
}

export interface SupportTicketAttachment {
  id: number;
  ticket_id: number;
  message_id?: number;
  uploader_id: number;
  original_name: string;
  mime_type: string;
  size: number;
  created_at: string;
}

export interface SupportTicketDetail {
  ticket: SupportTicket;
  messages: SupportTicketMessage[];
}

export interface SupportTicketListResponse {
  data: SupportTicket[];
  total: number;
  page: number;
  page_size: number;
  total_pages: number;
}

export interface SupportTicketListParams {
  status?: string;
  page?: number;
  page_size?: number;
}
