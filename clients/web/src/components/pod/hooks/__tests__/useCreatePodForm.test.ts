import { renderHook, act } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { getPodService, getEnvBundleService } from "@/lib/wasm-core";

vi.mock("@/stores/podCreation", () => ({
  usePodCreationStore: () => ({
    lastAgentSlug: null,
    lastRepositoryId: null,
    lastCredentialName: "",
    lastRuntimeBundleNames: [],
    lastBranchName: null,
    setLastChoices: vi.fn(),
    clearLastChoices: vi.fn(),
    _hasHydrated: true,
    setHasHydrated: vi.fn(),
  }),
}));

import { useCreatePodForm } from "../useCreatePodForm";

const mockAgents = [
  { name: "Claude Code", slug: "claude-code", is_builtin: true, is_active: true },
];

const mockCreatePod = vi.fn();
const mockListBundles = vi.fn();

function setupMocks() {
  // podState is a stable singleton — mutate its create_pod
  const podSvc = getPodService();
  (podSvc as unknown as Record<string, unknown>).create_pod = mockCreatePod;

  // EnvBundleService.list("credential", agent_slug) backs the form's bundle
  // selector; override the getter so each test can shape the response.
  vi.mocked(getEnvBundleService).mockReturnValue({
    list: mockListBundles,
  } as unknown as ReturnType<typeof getEnvBundleService>);
}

describe("useCreatePodForm - bundle via agentfile_layer (SSOT)", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    setupMocks();
    mockListBundles.mockResolvedValue(JSON.stringify({ items: [] }));
  });

  it("omits USE_ENV_BUNDLE from agentfile_layer when no bundle is selected", async () => {
    mockCreatePod.mockResolvedValue(
      JSON.stringify({ pod: { pod_key: "test-pod", id: 1, status: "initializing", agent_status: "idle" } })
    );

    const { result } = renderHook(() => useCreatePodForm(mockAgents, []));

    act(() => {
      result.current.setSelectedAgent("claude-code");
    });

    expect(result.current.selectedCredentialName).toBe("");
    expect(result.current.selectedRuntimeBundleNames).toEqual([]);

    await act(async () => {
      await result.current.submit(1, {}, { cols: 80, rows: 24 });
    });

    expect(mockCreatePod).toHaveBeenCalledTimes(1);
    const createArg = JSON.parse(mockCreatePod.mock.calls[0][0]);
    expect(createArg).not.toHaveProperty("credential_profile_id");
    const layer = createArg.agentfile_layer ?? "";
    expect(layer).not.toContain("USE_ENV_BUNDLE");
  });

  it("includes USE_ENV_BUNDLE — credential first then runtime in selection order", async () => {
    const credBundle = {
      id: 42, agent_slug: "claude-code", name: "My API Key",
      kind: "credential", kind_primary: false, is_active: true,
      created_at: "x", updated_at: "x",
    };
    const runtimeBundle = {
      id: 43, agent_slug: "claude-code", name: "production-debug",
      kind: "runtime", kind_primary: false, is_active: true,
      created_at: "x", updated_at: "x",
    };
    // useEnvBundles loads credential and runtime in parallel; first call =
    // credential, second = runtime. Order of mockResolvedValueOnce matters.
    mockListBundles.mockReset();
    mockListBundles.mockResolvedValueOnce(JSON.stringify({ items: [credBundle] }));
    mockListBundles.mockResolvedValueOnce(JSON.stringify({ items: [runtimeBundle] }));
    mockCreatePod.mockResolvedValue(
      JSON.stringify({ pod: { pod_key: "test-pod", id: 1, status: "initializing", agent_status: "idle" } })
    );

    const { result } = renderHook(() => useCreatePodForm(mockAgents, []));

    act(() => {
      result.current.setSelectedAgent("claude-code");
    });
    await act(async () => {});

    act(() => {
      result.current.setSelectedCredentialName("My API Key");
      result.current.setSelectedRuntimeBundleNames(["production-debug"]);
    });

    await act(async () => {
      await result.current.submit(1, {}, { cols: 80, rows: 24 });
    });

    expect(mockCreatePod).toHaveBeenCalledTimes(1);
    const createArg = JSON.parse(mockCreatePod.mock.calls[0][0]);
    expect(createArg).not.toHaveProperty("credential_profile_id");
    const layer: string = createArg.agentfile_layer ?? "";
    const useLines = layer.split("\n").filter((l) => l.startsWith("USE_ENV_BUNDLE"));
    // Credential always first, then runtime bundles.
    expect(useLines).toEqual([
      'USE_ENV_BUNDLE "My API Key"',
      'USE_ENV_BUNDLE "production-debug"',
    ]);
  });

  it("always sends agentfile_layer via API (SSOT)", async () => {
    mockCreatePod.mockResolvedValue(
      JSON.stringify({ pod: { pod_key: "test-pod", id: 1, status: "initializing", agent_status: "idle" } })
    );

    const { result } = renderHook(() => useCreatePodForm(mockAgents, []));

    act(() => {
      result.current.setSelectedAgent("claude-code");
    });

    await act(async () => {
      await result.current.submit(1, {}, { cols: 80, rows: 24 });
    });

    const createArg = JSON.parse(mockCreatePod.mock.calls[0][0]);
    expect(createArg).toHaveProperty("agent_slug", "claude-code");
    expect(createArg).not.toHaveProperty("credential_profile_id");
    expect(createArg).not.toHaveProperty("repository_id");
    expect(createArg).not.toHaveProperty("interaction_mode");
    expect(createArg).not.toHaveProperty("branch_name");
    expect(createArg).not.toHaveProperty("prompt");
    expect(createArg).not.toHaveProperty("config_overrides");
  });
});
