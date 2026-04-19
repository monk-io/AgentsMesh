import { initWasmCore, getSupportTicketService } from "@/lib/wasm-core";

export type {
  SupportTicket, SupportTicketMessage, SupportTicketAttachment,
  SupportTicketDetail, SupportTicketListResponse, SupportTicketListParams,
} from "./supportTicketTypes";

import type {
  SupportTicket, SupportTicketMessage, SupportTicketDetail,
  SupportTicketListResponse, SupportTicketListParams,
} from "./supportTicketTypes";

async function fileToUint8Array(file: File): Promise<Uint8Array> {
  return new Uint8Array(await file.arrayBuffer());
}

export async function createSupportTicket(data: {
  title: string; category: string; content: string; priority?: string; files?: File[];
}): Promise<SupportTicket> {
  await initWasmCore();
  const fileData = await Promise.all((data.files || []).map(fileToUint8Array));
  const fileNames = (data.files || []).map((f) => f.name);
  const json = await getSupportTicketService().create_ticket(
    data.title, data.category, data.content,
    data.priority ?? null, fileData, fileNames,
  );
  return JSON.parse(json);
}

export async function listSupportTickets(params?: SupportTicketListParams): Promise<SupportTicketListResponse> {
  await initWasmCore();
  return JSON.parse(await getSupportTicketService().list(
    params?.status ?? null, params?.page ?? null, params?.page_size ?? null,
  ));
}

export async function getSupportTicketDetail(id: number): Promise<SupportTicketDetail> {
  await initWasmCore();
  return JSON.parse(await getSupportTicketService().get_detail(BigInt(id)));
}

export async function addSupportTicketMessage(
  ticketId: number, content: string, files?: File[],
): Promise<SupportTicketMessage> {
  await initWasmCore();
  const fileData = await Promise.all((files || []).map(fileToUint8Array));
  const fileNames = (files || []).map((f) => f.name);
  const json = await getSupportTicketService().add_message(
    BigInt(ticketId), content, fileData, fileNames,
  );
  return JSON.parse(json);
}

export async function getSupportTicketAttachmentUrl(attachmentId: number): Promise<{ url: string }> {
  await initWasmCore();
  return JSON.parse(await getSupportTicketService().get_attachment_url(BigInt(attachmentId)));
}
