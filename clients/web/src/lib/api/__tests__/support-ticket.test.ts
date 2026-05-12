import { describe, it, expect, vi, beforeEach } from "vitest";
import { getSupportTicketService } from "@/lib/wasm-core";

if (!File.prototype.arrayBuffer) {
  File.prototype.arrayBuffer = function () {
    return new Promise((resolve) => {
      const reader = new FileReader();
      reader.onload = () => resolve(reader.result as ArrayBuffer);
      reader.readAsArrayBuffer(this);
    });
  };
}

// Hoisted mocks for the Connect adapter — support-ticket.ts now delegates
// list/getDetail/getAttachmentUrl to supportTicketConnect.ts after the
// proto migration. The legacy multipart paths (create_ticket /
// add_message) still hit the wasm bridge directly — see the dual-track
// note in support-ticket.ts.
const { listMock, getDetailMock, getAttUrlMock } = vi.hoisted(() => ({
  listMock: vi.fn(),
  getDetailMock: vi.fn(),
  getAttUrlMock: vi.fn(),
}));

vi.mock("../supportTicketConnect", () => ({
  listSupportTickets: listMock,
  getSupportTicketDetail: getDetailMock,
  getSupportTicketAttachmentUrl: getAttUrlMock,
}));

import {
  listSupportTickets, getSupportTicketDetail, addSupportTicketMessage,
  createSupportTicket, getSupportTicketAttachmentUrl,
} from "../support-ticket";

const mockCreate = vi.fn().mockResolvedValue('{"id":10}');
const mockAddMsg = vi.fn().mockResolvedValue('{"id":1,"content":"hello"}');

describe("support-ticket API (dual-track)", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(getSupportTicketService).mockReturnValue({
      create_ticket: mockCreate, add_message: mockAddMsg,
    } as unknown as ReturnType<typeof getSupportTicketService>);
    listMock.mockResolvedValue({
      data: [], total: 0, page: 1, page_size: 20, total_pages: 0,
    });
    getDetailMock.mockResolvedValue({
      ticket: {
        id: 42, user_id: 7, title: "x", category: "bug",
        status: "open", priority: "medium",
        created_at: "", updated_at: "",
      },
      messages: [],
    });
    getAttUrlMock.mockResolvedValue({
      url: "https://example.com/f.png",
    });
  });

  // ---- Migrated RPCs (Connect-RPC binary wire) ----

  it("listSupportTickets delegates to Connect adapter", async () => {
    await listSupportTickets({ status: "open", page: 2, page_size: 10 });
    expect(listMock).toHaveBeenCalledWith({ status: "open", page: 2, page_size: 10 });
  });

  it("listSupportTickets without params delegates to Connect adapter", async () => {
    await listSupportTickets();
    expect(listMock).toHaveBeenCalledWith(undefined);
  });

  it("getSupportTicketDetail delegates to Connect adapter", async () => {
    await getSupportTicketDetail(42);
    expect(getDetailMock).toHaveBeenCalledWith(42);
  });

  it("getSupportTicketAttachmentUrl delegates to Connect adapter", async () => {
    const result = await getSupportTicketAttachmentUrl(99);
    expect(getAttUrlMock).toHaveBeenCalledWith(99);
    expect(result).toEqual({ url: "https://example.com/f.png" });
  });

  // ---- Legacy REST multipart paths (dual-track) ----

  it("createSupportTicket with files still uses wasm bridge", async () => {
    const file = new File(["screenshot"], "shot.png", { type: "image/png" });
    await createSupportTicket({ title: "Bug", category: "bug", content: "Broke", priority: "high", files: [file] });
    expect(mockCreate).toHaveBeenCalledWith("Bug", "bug", "Broke", "high", [expect.any(Uint8Array)], ["shot.png"]);
  });

  it("createSupportTicket without optional fields still uses wasm bridge", async () => {
    await createSupportTicket({ title: "Q", category: "q", content: "How?" });
    expect(mockCreate).toHaveBeenCalledWith("Q", "q", "How?", null, [], []);
  });

  it("addSupportTicketMessage with files still uses wasm bridge", async () => {
    const file = new File(["data"], "f.txt", { type: "text/plain" });
    await addSupportTicketMessage(5, "hello", [file]);
    expect(mockAddMsg).toHaveBeenCalledWith(BigInt(5), "hello", [expect.any(Uint8Array)], ["f.txt"]);
  });

  it("addSupportTicketMessage without files still uses wasm bridge", async () => {
    await addSupportTicketMessage(5, "hello");
    expect(mockAddMsg).toHaveBeenCalledWith(BigInt(5), "hello", [], []);
  });
});
