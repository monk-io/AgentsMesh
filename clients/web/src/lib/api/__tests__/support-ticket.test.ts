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

import {
  listSupportTickets, getSupportTicketDetail, addSupportTicketMessage,
  createSupportTicket, getSupportTicketAttachmentUrl,
} from "../support-ticket";

const mockList = vi.fn().mockResolvedValue('{"data":[],"total":0}');
const mockGetDetail = vi.fn().mockResolvedValue('{"ticket":{},"messages":[]}');
const mockGetAttUrl = vi.fn().mockResolvedValue('{"url":"https://example.com/f.png"}');
const mockCreate = vi.fn().mockResolvedValue('{"id":10}');
const mockAddMsg = vi.fn().mockResolvedValue('{"id":1,"content":"hello"}');

describe("support-ticket API (WASM service)", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(getSupportTicketService).mockReturnValue({
      list: mockList, get_detail: mockGetDetail, get_attachment_url: mockGetAttUrl,
      create_ticket: mockCreate, add_message: mockAddMsg,
    } as unknown as ReturnType<typeof getSupportTicketService>);
  });

  it("listSupportTickets with params", async () => {
    await listSupportTickets({ status: "open", page: 2, page_size: 10 });
    expect(mockList).toHaveBeenCalledWith("open", 2, 10);
  });

  it("listSupportTickets without params", async () => {
    await listSupportTickets();
    expect(mockList).toHaveBeenCalledWith(null, null, null);
  });

  it("getSupportTicketDetail", async () => {
    await getSupportTicketDetail(42);
    expect(mockGetDetail).toHaveBeenCalledWith(BigInt(42));
  });

  it("createSupportTicket with files", async () => {
    const file = new File(["screenshot"], "shot.png", { type: "image/png" });
    await createSupportTicket({ title: "Bug", category: "bug", content: "Broke", priority: "high", files: [file] });
    expect(mockCreate).toHaveBeenCalledWith("Bug", "bug", "Broke", "high", [expect.any(Uint8Array)], ["shot.png"]);
  });

  it("createSupportTicket without optional fields", async () => {
    await createSupportTicket({ title: "Q", category: "q", content: "How?" });
    expect(mockCreate).toHaveBeenCalledWith("Q", "q", "How?", null, [], []);
  });

  it("addSupportTicketMessage with files", async () => {
    const file = new File(["data"], "f.txt", { type: "text/plain" });
    await addSupportTicketMessage(5, "hello", [file]);
    expect(mockAddMsg).toHaveBeenCalledWith(BigInt(5), "hello", [expect.any(Uint8Array)], ["f.txt"]);
  });

  it("addSupportTicketMessage without files", async () => {
    await addSupportTicketMessage(5, "hello");
    expect(mockAddMsg).toHaveBeenCalledWith(BigInt(5), "hello", [], []);
  });

  it("getSupportTicketAttachmentUrl", async () => {
    const result = await getSupportTicketAttachmentUrl(99);
    expect(mockGetAttUrl).toHaveBeenCalledWith(BigInt(99));
    expect(result).toEqual({ url: "https://example.com/f.png" });
  });
});
