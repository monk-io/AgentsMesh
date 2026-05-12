// Legacy support-ticket adapter. After the proto migration the four
// JSON-bodied operations (list, getDetail, getAttachmentUrl) delegate to
// supportTicketConnect.ts (binary-wire Connect-RPC) — see migration ADR.
//
// Two operations remain on REST during dual-track:
//   * createSupportTicket — multipart/form-data with optional files[]
//   * addSupportTicketMessage — multipart/form-data with optional files[]
// Connect-RPC has no multipart story; these stay until a follow-up
// chunked-upload Connect path lands or the file upload surface gets
// extracted to its own RPC + presigned URL flow.
//
// New call sites should import from `./supportTicketConnect` for the
// migrated RPCs; this module stays as the dual-track shim.

import { initWasmCore, getSupportTicketService } from "@/lib/wasm-core";
import {
  getSupportTicketAttachmentUrl as getAttachmentUrlConnect,
  getSupportTicketDetail as getDetailConnect,
  listSupportTickets as listConnect,
} from "./supportTicketConnect";

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

export async function listSupportTickets(
  params?: SupportTicketListParams,
): Promise<SupportTicketListResponse> {
  await initWasmCore();
  return listConnect(params);
}

export async function getSupportTicketDetail(id: number): Promise<SupportTicketDetail> {
  await initWasmCore();
  return getDetailConnect(id);
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

export async function getSupportTicketAttachmentUrl(
  attachmentId: number,
): Promise<{ url: string }> {
  await initWasmCore();
  return getAttachmentUrlConnect(attachmentId);
}
