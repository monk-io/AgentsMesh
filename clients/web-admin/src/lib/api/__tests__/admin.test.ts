import { describe, it, expect, vi, beforeEach } from "vitest";

// Users / Organizations / Runners migrated to Connect-RPC — see
// adminUsers.test.ts, adminOrganizations.test.ts, and adminRunners.test.ts.
// The rest (Dashboard / Audit Logs / Auth) still routes through REST apiClient.
const mockGet = vi.fn();
const mockPost = vi.fn();
const mockDelete = vi.fn();

vi.mock("../base", () => ({
  apiClient: {
    get: (...args: unknown[]) => mockGet(...args),
    post: (...args: unknown[]) => mockPost(...args),
    put: (...args: unknown[]) => vi.fn()(...args),
    delete: (...args: unknown[]) => mockDelete(...args),
  },
}));

import {
  getDashboardStats,
  listAuditLogs,
  login,
  getCurrentAdmin,
} from "../admin";

describe("Admin API - REST surface", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe("Dashboard", () => {
    it("getDashboardStats calls GET /dashboard/stats", async () => {
      mockGet.mockResolvedValue({ total_users: 100 });
      const result = await getDashboardStats();
      expect(mockGet).toHaveBeenCalledWith("/dashboard/stats");
      expect(result.total_users).toBe(100);
    });
  });

  describe("Audit Logs", () => {
    it("listAuditLogs calls GET /audit-logs with params", async () => {
      mockGet.mockResolvedValue({ data: [], total: 0 });
      await listAuditLogs({ target_type: "user", page: 1 });
      expect(mockGet).toHaveBeenCalledWith(
        "/audit-logs",
        expect.objectContaining({ target_type: "user", page: 1 }),
      );
    });
  });

  describe("Auth", () => {
    it("login calls POST /auth/login", async () => {
      mockPost.mockResolvedValue({ token: "t", user: {} });
      await login({ email: "admin@test.com", password: "pass" });
      expect(mockPost).toHaveBeenCalledWith("/auth/login", {
        email: "admin@test.com",
        password: "pass",
      });
    });

    it("getCurrentAdmin calls GET /me", async () => {
      mockGet.mockResolvedValue({ id: 1 });
      await getCurrentAdmin();
      expect(mockGet).toHaveBeenCalledWith("/me");
    });
  });
});
