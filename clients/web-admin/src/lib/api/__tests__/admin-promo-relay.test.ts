import { describe, it, expect, vi, beforeEach } from "vitest";

// Mock the apiClient (promo codes still use it)
const mockGet = vi.fn();
const mockPost = vi.fn();
const mockPut = vi.fn();
const mockDelete = vi.fn();

vi.mock("../base", () => ({
  apiClient: {
    get: (...args: unknown[]) => mockGet(...args),
    post: (...args: unknown[]) => mockPost(...args),
    put: (...args: unknown[]) => mockPut(...args),
    delete: (...args: unknown[]) => mockDelete(...args),
  },
}));

// Mock Connect transport (relays migrated to Connect-RPC)
const mockCallConnect = vi.fn();
vi.mock("@/lib/connect/transport", () => ({
  callConnect: (...args: unknown[]) => mockCallConnect(...args),
  ConnectError: class ConnectError extends Error {
    code: string;
    status: number;
    constructor(msg: string, code: string, status: number) {
      super(msg);
      this.code = code;
      this.status = status;
    }
  },
}));

import {
  listPromoCodes,
  getPromoCode,
  createPromoCode,
  updatePromoCode,
  activatePromoCode,
  deactivatePromoCode,
  deletePromoCode,
  listPromoCodeRedemptions,
  listRelays,
  getRelayStats,
  getRelay,
  forceUnregisterRelay,
  listSessions,
  migrateSession,
  bulkMigrateSessions,
} from "../admin";

describe("Admin API - Promo Codes & Relays", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe("Promo Codes", () => {
    it("listPromoCodes converts boolean is_active to string", async () => {
      mockGet.mockResolvedValue({ data: [], total: 0 });
      await listPromoCodes({ is_active: true, type: "media" });
      expect(mockGet).toHaveBeenCalledWith(
        "/promo-codes",
        expect.objectContaining({ is_active: "true", type: "media" })
      );
    });

    it("listPromoCodes omits undefined params", async () => {
      mockGet.mockResolvedValue({ data: [], total: 0 });
      await listPromoCodes({ page: 1 });
      const params = mockGet.mock.calls[0][1];
      expect(params.search).toBeUndefined();
      expect(params.type).toBeUndefined();
    });

    it("getPromoCode calls GET /promo-codes/:id", async () => {
      mockGet.mockResolvedValue({ id: 10 });
      await getPromoCode(10);
      expect(mockGet).toHaveBeenCalledWith("/promo-codes/10");
    });

    it("createPromoCode calls POST /promo-codes", async () => {
      const data = {
        code: "TEST50",
        name: "Test",
        type: "campaign" as const,
        plan_name: "pro",
        duration_months: 3,
      };
      mockPost.mockResolvedValue({ id: 1, ...data });
      await createPromoCode(data);
      expect(mockPost).toHaveBeenCalledWith("/promo-codes", data);
    });

    it("activatePromoCode calls POST /promo-codes/:id/activate", async () => {
      mockPost.mockResolvedValue({ message: "ok" });
      await activatePromoCode(5);
      expect(mockPost).toHaveBeenCalledWith("/promo-codes/5/activate");
    });

    it("deactivatePromoCode calls POST /promo-codes/:id/deactivate", async () => {
      mockPost.mockResolvedValue({ message: "ok" });
      await deactivatePromoCode(5);
      expect(mockPost).toHaveBeenCalledWith("/promo-codes/5/deactivate");
    });

    it("deletePromoCode calls DELETE /promo-codes/:id", async () => {
      mockDelete.mockResolvedValue({ message: "ok" });
      await deletePromoCode(5);
      expect(mockDelete).toHaveBeenCalledWith("/promo-codes/5");
    });

    it("updatePromoCode calls PUT /promo-codes/:id", async () => {
      mockPut.mockResolvedValue({ id: 5, name: "Updated" });
      await updatePromoCode(5, { name: "Updated" });
      expect(mockPut).toHaveBeenCalledWith("/promo-codes/5", { name: "Updated" });
    });

    it("listPromoCodeRedemptions calls GET /promo-codes/:id/redemptions", async () => {
      mockGet.mockResolvedValue({ data: [], total: 0 });
      await listPromoCodeRedemptions(5, { page: 1, page_size: 10 });
      expect(mockGet).toHaveBeenCalledWith(
        "/promo-codes/5/redemptions",
        expect.objectContaining({ page: 1, page_size: 10 })
      );
    });

    it("listPromoCodes converts is_active false to string", async () => {
      mockGet.mockResolvedValue({ data: [], total: 0 });
      await listPromoCodes({ is_active: false });
      const params = mockGet.mock.calls[0][1];
      expect(params.is_active).toBe("false");
    });

    it("listPromoCodes with all params set", async () => {
      mockGet.mockResolvedValue({ data: [], total: 0 });
      await listPromoCodes({
        search: "TEST",
        type: "campaign",
        plan_name: "pro",
        is_active: true,
        page: 2,
        page_size: 25,
      });
      expect(mockGet).toHaveBeenCalledWith(
        "/promo-codes",
        expect.objectContaining({
          search: "TEST",
          type: "campaign",
          plan_name: "pro",
          is_active: "true",
          page: 2,
          page_size: 25,
        })
      );
    });
  });

  describe("Relays (Connect-RPC)", () => {
    it("listRelays calls AdminService.ListRelays and converts to snake_case", async () => {
      mockCallConnect.mockResolvedValue({
        items: [
          {
            id: "relay-1",
            url: "wss://relay1.example.com",
            region: "us-east",
            capacity: 100,
            connections: 5,
            cpuUsage: 12.5,
            memoryUsage: 30.2,
            lastHeartbeat: "2026-01-01T00:00:00Z",
            healthy: true,
          },
        ],
        total: 1,
      });
      const out = await listRelays();
      expect(mockCallConnect.mock.calls[0][0]).toBe("proto.admin.v1.AdminService");
      expect(mockCallConnect.mock.calls[0][1]).toBe("ListRelays");
      expect(out.total).toBe(1);
      expect(out.data[0]).toMatchObject({
        id: "relay-1",
        cpu_usage: 12.5,
        memory_usage: 30.2,
        last_heartbeat: "2026-01-01T00:00:00Z",
      });
    });

    it("getRelayStats calls AdminService.GetRelayStats", async () => {
      mockCallConnect.mockResolvedValue({
        totalRelays: 3,
        healthyRelays: 2,
        totalConnections: 25,
      });
      const out = await getRelayStats();
      expect(mockCallConnect.mock.calls[0][1]).toBe("GetRelayStats");
      expect(out).toEqual({
        total_relays: 3,
        healthy_relays: 2,
        total_connections: 25,
        active_sessions: 0,
      });
    });

    it("forceUnregisterRelay sends relay id and returns normalized shape", async () => {
      mockCallConnect.mockResolvedValue({
        status: "unregistered",
        relayId: "relay/special",
      });
      const out = await forceUnregisterRelay("relay/special", true);
      expect(mockCallConnect.mock.calls[0][1]).toBe("ForceUnregisterRelay");
      expect(mockCallConnect.mock.calls[0][4]).toEqual({ id: "relay/special" });
      expect(out).toEqual({
        status: "unregistered",
        relay_id: "relay/special",
        affected_sessions: 0,
      });
    });

    it("getRelay sends id and synthesizes empty sessions", async () => {
      mockCallConnect.mockResolvedValue({
        relay: {
          id: "relay/with-slash",
          url: "wss://r.example.com",
          region: "",
          capacity: 0,
          connections: 0,
          cpuUsage: 0,
          memoryUsage: 0,
          lastHeartbeat: "",
          healthy: false,
        },
      });
      const out = await getRelay("relay/with-slash");
      expect(mockCallConnect.mock.calls[0][1]).toBe("GetRelay");
      expect(mockCallConnect.mock.calls[0][4]).toEqual({ id: "relay/with-slash" });
      expect(out.session_count).toBe(0);
      expect(out.sessions).toEqual([]);
      expect(out.relay.id).toBe("relay/with-slash");
    });

    it("getRelay throws when proto omits relay payload", async () => {
      mockCallConnect.mockResolvedValue({});
      await expect(getRelay("relay-x")).rejects.toThrow(/Relay not found/);
    });

    it("listSessions throws unimplemented", async () => {
      await expect(listSessions("relay-1")).rejects.toThrow(/listSessions/);
    });

    it("migrateSession throws unimplemented", async () => {
      await expect(migrateSession("pod-1", "relay-2")).rejects.toThrow(/migrateSession/);
    });

    it("bulkMigrateSessions throws unimplemented", async () => {
      await expect(bulkMigrateSessions("relay-1", "relay-2")).rejects.toThrow(
        /bulkMigrateSessions/,
      );
    });

    it("forceUnregisterRelay defaults migrateSessions to false (param dropped on Connect)", async () => {
      mockCallConnect.mockResolvedValue({ status: "unregistered", relayId: "relay-1" });
      await forceUnregisterRelay("relay-1");
      expect(mockCallConnect.mock.calls[0][4]).toEqual({ id: "relay-1" });
    });
  });
});
