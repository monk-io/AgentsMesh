import { renderHook, act, waitFor } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { getUserCredentialService } from "@/lib/wasm-core";
import * as agentConnect from "@/lib/api/agentConnect";

const mockListCredentials = vi.fn();
const mockCreateCredential = vi.fn();
const mockUpdateCredential = vi.fn();

const stable = vi.hoisted(() => ({
  org: { id: 1, name: "TestOrg", slug: "test-org" },
  user: { id: 1, email: "u@e.com", username: "u" },
}));

vi.mock("@/stores/auth", () => ({
  useCurrentOrg: () => stable.org,
  useCurrentUser: () => stable.user,
  useAuthOrganizations: () => [],
  useAuthStore: () => ({ currentOrg: stable.org }),
  useIsAuthenticated: () => true,
  readCurrentUser: () => stable.user,
  readCurrentOrg: () => stable.org,
  readOrganizations: () => [],
}));

vi.mock("@/lib/api/agentConnect", () => ({
  listAgents: vi.fn(),
  getAgentConfigSchema: vi.fn(),
}));

import { useAgentCredentials } from "../useAgentCredentials";
import type { CredentialFormData } from "../types";

const mockTranslate = (key: string) => key;

describe("useAgentCredentials - handleSaveProfile error handling", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockListCredentials.mockResolvedValue(JSON.stringify({ items: [] }));
    vi.mocked(agentConnect.listAgents).mockResolvedValue({
      items: [{ name: "Claude", slug: "claude-code", description: "", is_active: true, sort_order: 0, supports_acp: false, supports_pty: true, supports_built_in_mcp: false }],
      total: 1,
      limit: 0,
      offset: 0,
    });
    vi.mocked(agentConnect.getAgentConfigSchema).mockResolvedValue({
      schema: {
        fields: [],
        credential_fields: [
          { name: "ANTHROPIC_API_KEY", type: "secret", optional: true },
          { name: "ANTHROPIC_AUTH_TOKEN", type: "secret", optional: true },
          { name: "ANTHROPIC_BASE_URL", type: "text", optional: true },
        ],
      },
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } as any);

    vi.mocked(getUserCredentialService).mockReturnValue({
      ...getUserCredentialService(),
      list_agent_credentials: mockListCredentials,
      create_agent_credential: mockCreateCredential,
      update_agent_credential: mockUpdateCredential,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } as any);
  });

  it("should propagate API errors from create to caller", async () => {
    const apiError = new Error("Network error");
    mockCreateCredential.mockRejectedValue(apiError);

    const { result } = renderHook(() => useAgentCredentials(mockTranslate));

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    const formData: CredentialFormData = {
      name: "Test Profile",
      description: "",
      credentials: { ANTHROPIC_API_KEY: "sk-test" },
    };

    await expect(
      act(async () => {
        await result.current.handleSaveProfile("claude-code", formData, null);
      })
    ).rejects.toThrow("Network error");
  });

  it("should propagate API errors from update to caller", async () => {
    const apiError = new Error("Unauthorized");
    mockUpdateCredential.mockRejectedValue(apiError);

    const { result } = renderHook(() => useAgentCredentials(mockTranslate));

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    const editingProfile = {
      id: 5,
      user_id: 1,
      agent_slug: "claude-code",
      name: "Existing",
      is_runner_host: false,
      is_default: false,
      is_active: true,
      created_at: "2024-01-01",
      updated_at: "2024-01-01",
    };

    const formData: CredentialFormData = {
      name: "Updated",
      description: "",
      credentials: { ANTHROPIC_API_KEY: "sk-new" },
    };

    await expect(
      act(async () => {
        await result.current.handleSaveProfile("claude-code", formData, editingProfile);
      })
    ).rejects.toThrow("Unauthorized");
  });

  it("should set success message and call loadData on successful create", async () => {
    mockCreateCredential.mockResolvedValue(JSON.stringify({ profile: { id: 1 } }));

    const { result } = renderHook(() => useAgentCredentials(mockTranslate));

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    const formData: CredentialFormData = {
      name: "New Profile",
      description: "",
      credentials: { ANTHROPIC_API_KEY: "sk-test" },
    };

    await act(async () => {
      await result.current.handleSaveProfile("claude-code", formData, null);
    });

    expect(mockCreateCredential).toHaveBeenCalledTimes(1);
    expect(result.current.success).toBe("settings.agentCredentials.profileCreated");
    expect(mockListCredentials).toHaveBeenCalledTimes(2);
  });
});
