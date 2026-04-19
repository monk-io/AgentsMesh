import { invoke } from "./invoke";
import type { ISupportTicketService } from "@agentsmesh/service-interface";

export class ElectronSupportTicketService implements ISupportTicketService {
  async list(status?: string | null, page?: number | null, pageSize?: number | null): Promise<string> {
    return invoke<string>("supportTicketList", status, page, pageSize);
  }

  async get_detail(id: bigint): Promise<string> {
    return invoke<string>("supportTicketGetDetail", Number(id));
  }

  async create_ticket(title: string, category: string, content: string, priority: string | null | undefined, fileData: Uint8Array[], fileNames: string[]): Promise<string> {
    return invoke<string>("supportTicketCreateTicket", title, category, content, priority, fileData.map(d => Array.from(d)), fileNames);
  }

  async add_message(ticketId: bigint, content: string, fileData: Uint8Array[], fileNames: string[]): Promise<string> {
    return invoke<string>("supportTicketAddMessage", Number(ticketId), content, fileData.map(d => Array.from(d)), fileNames);
  }

  async get_attachment_url(id: bigint): Promise<string> {
    return invoke<string>("supportTicketGetAttachmentUrl", Number(id));
  }
}
