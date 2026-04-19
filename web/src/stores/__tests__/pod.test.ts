import { describe, it, expect, beforeEach, vi } from "vitest";
import { act } from "@testing-library/react";
import { usePodStore } from "../pod";
import { getPodService } from "@/lib/wasm-core";
import { mockPod, mockPod2, resetPodStore, seedPods, readPods, readCurrentPod } from "./pod-test-utils";

function svc() {
  return getPodService() as unknown as {
    fetch_pods: ReturnType<typeof vi.fn>;
    fetch_pod: ReturnType<typeof vi.fn>;
    set_pods: (json: string) => void;
    upsert_pod: (json: string) => void;
  };
}

function mockFetchPodsResponse(pods: unknown[], total = pods.length) {
  vi.mocked(svc().fetch_pods).mockImplementation(async () => {
    svc().set_pods(JSON.stringify(pods));
    return JSON.stringify({ pods, total });
  });
}

function mockFetchPodOk(pod: unknown) {
  vi.mocked(svc().fetch_pod).mockImplementation(async () => {
    svc().upsert_pod(JSON.stringify(pod));
    return JSON.stringify(pod);
  });
}

describe("Pod Store — basic reads", () => {
  beforeEach(resetPodStore);

  describe("initial state", () => {
    it("should have default values", () => {
      const state = usePodStore.getState();
      expect(readPods()).toEqual([]);
      expect(readCurrentPod()).toBeNull();
      expect(state.loading).toBe(false);
      expect(state.error).toBeNull();
    });
  });

  describe("fetchPods", () => {
    it("should fetch pods successfully", async () => {
      mockFetchPodsResponse([mockPod, mockPod2], 2);

      await act(async () => { await usePodStore.getState().fetchPods(); });

      const pods = readPods();
      expect(pods).toHaveLength(2);
      expect(pods[0].pod_key).toBe("pod-abc-123");
      expect(usePodStore.getState().loading).toBe(false);
      expect(usePodStore.getState().error).toBeNull();
    });

    it("should pass filters to WASM service", async () => {
      mockFetchPodsResponse([]);

      await act(async () => {
        await usePodStore.getState().fetchPods({ status: "running", runnerId: 1 });
      });

      expect(svc().fetch_pods).toHaveBeenCalledWith("running", BigInt(1), null, null, null);
    });

    it("should handle empty response", async () => {
      mockFetchPodsResponse([]);
      await act(async () => { await usePodStore.getState().fetchPods(); });
      expect(readPods()).toEqual([]);
    });

    it("should handle fetch error", async () => {
      vi.mocked(svc().fetch_pods).mockRejectedValue(new Error("Network error"));

      await act(async () => { await usePodStore.getState().fetchPods(); });

      const state = usePodStore.getState();
      expect(state.error).toBe("Network error");
      expect(state.loading).toBe(false);
    });

    it("should use default error message when no message provided", async () => {
      vi.mocked(svc().fetch_pods).mockRejectedValue({});

      await act(async () => { await usePodStore.getState().fetchPods(); });
      expect(usePodStore.getState().error).toBe("Failed to fetch pods");
    });
  });

  describe("fetchPod", () => {
    it("should fetch single pod successfully", async () => {
      mockFetchPodOk(mockPod);

      await act(async () => { await usePodStore.getState().fetchPod("pod-abc-123"); });

      expect(readCurrentPod()).toBeNull();
      expect(usePodStore.getState().loading).toBe(false);
    });

    it("should add fetched pod to pods array when not present", async () => {
      mockFetchPodOk(mockPod);
      expect(readPods()).toEqual([]);

      await act(async () => { await usePodStore.getState().fetchPod("pod-abc-123"); });

      const pods = readPods();
      expect(pods).toHaveLength(1);
      expect(pods[0]).toEqual(mockPod);
    });

    it("should update existing pod in pods array when present", async () => {
      const updatedPod = { ...mockPod, status: "terminated" as const };
      seedPods(mockPod, mockPod2);
      mockFetchPodOk(updatedPod);

      await act(async () => { await usePodStore.getState().fetchPod("pod-abc-123"); });

      const pods = readPods();
      expect(pods).toHaveLength(2);
      expect(pods.find(p => p.pod_key === "pod-abc-123")?.status).toBe("terminated");
      expect(pods.find(p => p.pod_key === "pod-def-456")).toEqual(mockPod2);
    });

    it("should handle fetch error", async () => {
      vi.mocked(svc().fetch_pod).mockRejectedValue({ message: "Pod not found" });

      await act(async () => {
        await usePodStore.getState().fetchPod("non-existent").catch(() => {});
      });

      const state = usePodStore.getState();
      expect(state.error).toBeNull();
      expect(state.loading).toBe(false);
    });
  });

  describe("setCurrentPod", () => {
    it("should set current pod", () => {
      act(() => { usePodStore.getState().setCurrentPod(mockPod); });
      expect(readCurrentPod()).toEqual(mockPod);
    });

    it("should set to null", () => {
      usePodStore.getState().setCurrentPod(mockPod);
      act(() => { usePodStore.getState().setCurrentPod(null); });
      expect(readCurrentPod()).toBeNull();
    });
  });

  describe("clearError", () => {
    it("should clear error", () => {
      usePodStore.setState({ error: "Some error" });
      act(() => { usePodStore.getState().clearError(); });
      expect(usePodStore.getState().error).toBeNull();
    });
  });
});
