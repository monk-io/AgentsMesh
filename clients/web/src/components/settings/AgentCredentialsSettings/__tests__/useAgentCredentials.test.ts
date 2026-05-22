import { renderHook, act, waitFor } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { getEnvBundleService, getAgentService } from "@/lib/wasm-core";

const mockList = vi.fn();
const mockListAgents = vi.fn();
const mockCreate = vi.fn();
const mockUpdate = vi.fn();

import { useAgentCredentials } from "../useAgentCredentials";
import type { CredentialFormData } from "../types";

const mockTranslate = (key: string) => key;

describe("useAgentCredentials - handleSaveProfile error handling", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockList.mockResolvedValue(JSON.stringify({ items: [] }));
    mockListAgents.mockResolvedValue(
      JSON.stringify({ agents: [{ name: "Claude", slug: "claude-code" }] })
    );

    vi.mocked(getEnvBundleService).mockReturnValue({
      ...getEnvBundleService(),
      list: mockList,
      create: mockCreate,
      update: mockUpdate,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } as any);

    vi.mocked(getAgentService).mockReturnValue({
      ...getAgentService(),
      list_agents: mockListAgents,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } as any);
  });

  it("should propagate API errors from create to caller", async () => {
    mockCreate.mockRejectedValue(new Error("Network error"));

    const { result } = renderHook(() => useAgentCredentials(mockTranslate));
    await waitFor(() => expect(result.current.loading).toBe(false));

    const formData: CredentialFormData = {
      name: "Test Bundle",
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
    mockUpdate.mockRejectedValue(new Error("Unauthorized"));

    const { result } = renderHook(() => useAgentCredentials(mockTranslate));
    await waitFor(() => expect(result.current.loading).toBe(false));

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
    mockCreate.mockResolvedValue(JSON.stringify({ id: 1 }));

    const { result } = renderHook(() => useAgentCredentials(mockTranslate));
    await waitFor(() => expect(result.current.loading).toBe(false));

    const formData: CredentialFormData = {
      name: "New Bundle",
      description: "",
      credentials: { ANTHROPIC_API_KEY: "sk-test" },
    };

    await act(async () => {
      await result.current.handleSaveProfile("claude-code", formData, null);
    });

    expect(mockCreate).toHaveBeenCalledTimes(1);
    expect(result.current.success).toBe("settings.agentCredentials.profileCreated");
    // Loaded once at mount + once after save.
    expect(mockList).toHaveBeenCalledTimes(2);
  });
});
