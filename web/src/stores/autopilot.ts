import { create } from "zustand";
import { useMemo } from "react";
import type {
  AutopilotControllerData, AutopilotIterationData,
  CreateAutopilotControllerRequest, ApproveRequest,
} from "@/lib/api/autopilotTypes";
import { AutopilotThinkingData } from "@/lib/realtime/types";
import { reconnectRegistry } from "@/lib/realtime";
import { getErrorMessage } from "@/lib/utils";
import { getAutopilotService, parseWasmAny } from "@/lib/wasm-core";

export type AutopilotController = AutopilotControllerData;
export type AutopilotIteration = AutopilotIterationData;
export type AutopilotThinking = AutopilotThinkingData;
export type { CreateAutopilotControllerRequest, ApproveRequest };

type Ctrl = AutopilotController;
const ACTIVE = ["initializing", "running", "paused", "user_takeover", "waiting_approval"];
const svc = () => getAutopilotService();
const bump = () => useAutopilotStore.setState((s) => ({ _tick: s._tick + 1 }));

function updateCtrlInWasm(key: string, updater: (c: Ctrl) => Ctrl) {
  const ctrls: Ctrl[] = JSON.parse(svc().controllers_json());
  const target = ctrls.find((c) => c.autopilot_controller_key === key);
  if (target) svc().update_controller(key, JSON.stringify(updater(target)));
}

export function useAutopilotControllers(): Ctrl[] {
  const tick = useAutopilotStore((s) => s._tick);
  return useMemo(() => JSON.parse(svc().controllers_json()), [tick]);
}

export function useCurrentAutopilotController(): Ctrl | null {
  const tick = useAutopilotStore((s) => s._tick);
  return useMemo(() => parseWasmAny<Ctrl>(svc().current_controller_json()), [tick]);
}

interface AutopilotState {
  _tick: number;
  iterations: Record<string, AutopilotIteration[]>;
  thinking: Record<string, AutopilotThinking | null>;
  thinkingHistory: Record<string, AutopilotThinking[]>;
  loading: boolean; error: string | null;
}

interface AutopilotActions {
  fetchAutopilotControllers: () => Promise<void>;
  fetchAutopilotController: (key: string) => Promise<void>;
  createAutopilotController: (data: CreateAutopilotControllerRequest) => Promise<Ctrl>;
  pauseAutopilotController: (key: string) => Promise<void>;
  resumeAutopilotController: (key: string) => Promise<void>;
  stopAutopilotController: (key: string) => Promise<void>;
  approveAutopilotController: (key: string, data?: ApproveRequest) => Promise<void>;
  takeoverAutopilotController: (key: string) => Promise<void>;
  handbackAutopilotController: (key: string) => Promise<void>;
  fetchIterations: (key: string) => Promise<void>;
  updateAutopilotControllerStatus: (key: string, phase: string, cur: number, max: number, cbState: string, cbReason?: string) => void;
  addIteration: (key: string, iteration: AutopilotIteration) => void;
  updateThinking: (key: string, thinking: AutopilotThinking) => void;
  setCurrentAutopilotController: (c: Ctrl | null) => void;
  removeAutopilotController: (key: string) => void;
  clearError: () => void;
  getAutopilotControllerByPodKey: (podKey: string) => Ctrl | undefined;
  getThinking: (key: string) => AutopilotThinking | null;
  getThinkingHistory: (key: string) => AutopilotThinking[];
}

export const useAutopilotStore = create<AutopilotState & AutopilotActions>((set, get) => ({
  _tick: 0, iterations: {}, thinking: {}, thinkingHistory: {}, loading: false, error: null,

  fetchAutopilotControllers: async () => {
    set({ loading: true, error: null });
    try { await svc().fetch_controllers(); set({ loading: false, _tick: get()._tick + 1 }); }
    catch (e) { set({ error: getErrorMessage(e, "Failed to fetch controllers"), loading: false }); }
  },

  fetchAutopilotController: async (key) => {
    try { await svc().fetch_controller(key); bump(); }
    catch (e) { set({ error: getErrorMessage(e, "Failed to fetch controller") }); }
  },

  createAutopilotController: async (data) => {
    try {
      const json = await svc().create_controller(JSON.stringify(data));
      const c: Ctrl = JSON.parse(json);
      bump();
      return c;
    } catch (e) { set({ error: getErrorMessage(e, "Failed to create") }); throw e; }
  },

  pauseAutopilotController: async (key) => {
    try { await svc().pause_controller(key); updateCtrlInWasm(key, (c) => ({ ...c, phase: "paused" })); bump(); }
    catch (e) { set({ error: getErrorMessage(e, "Failed to pause") }); }
  },

  resumeAutopilotController: async (key) => {
    try { await svc().resume_controller(key); updateCtrlInWasm(key, (c) => ({ ...c, phase: "running" })); bump(); }
    catch (e) { set({ error: getErrorMessage(e, "Failed to resume") }); }
  },

  stopAutopilotController: async (key) => {
    try { await svc().stop_controller(key); updateCtrlInWasm(key, (c) => ({ ...c, phase: "stopped" })); bump(); }
    catch (e) { set({ error: getErrorMessage(e, "Failed to stop") }); }
  },

  approveAutopilotController: async (key, data) => {
    try {
      await svc().approve_controller(key, JSON.stringify(data || {}));
      updateCtrlInWasm(key, (c) => ({
        ...c, phase: data?.continue_execution === false ? "stopped" : "running",
        max_iterations: data?.additional_iterations ? c.max_iterations + data.additional_iterations : c.max_iterations,
      }));
      bump();
    } catch (e) { set({ error: getErrorMessage(e, "Failed to approve") }); }
  },

  takeoverAutopilotController: async (key) => {
    try { await svc().takeover_controller(key); updateCtrlInWasm(key, (c) => ({ ...c, phase: "user_takeover", user_takeover: true })); bump(); }
    catch (e) { set({ error: getErrorMessage(e, "Failed to takeover") }); }
  },

  handbackAutopilotController: async (key) => {
    try { await svc().handback_controller(key); updateCtrlInWasm(key, (c) => ({ ...c, phase: "running", user_takeover: false })); bump(); }
    catch (e) { set({ error: getErrorMessage(e, "Failed to handback") }); }
  },

  fetchIterations: async (key) => {
    try {
      await svc().fetch_iterations(key);
      const parsed = parseWasmAny<AutopilotIteration[]>(svc().get_iterations_json(key));
      set((s) => ({ iterations: { ...s.iterations, [key]: parsed || [] } }));
    } catch (e) { set({ error: getErrorMessage(e, "Failed to fetch iterations") }); }
  },

  updateAutopilotControllerStatus: (key, phase, cur, max, cbState, cbReason) => {
    updateCtrlInWasm(key, (c) => ({
      ...c, phase: phase as Ctrl["phase"], current_iteration: cur, max_iterations: max,
      circuit_breaker: { state: cbState as Ctrl["circuit_breaker"]["state"], reason: cbReason },
    }));
    bump();
  },

  addIteration: (key, iter) => {
    svc().add_iteration(key, JSON.stringify(iter));
    const parsed = parseWasmAny<AutopilotIteration[]>(svc().get_iterations_json(key));
    set((s) => ({ iterations: { ...s.iterations, [key]: parsed || [] } }));
  },

  updateThinking: (key, t) => {
    svc().update_thinking(key, JSON.stringify(t));
    const thinking = parseWasmAny<AutopilotThinking>(svc().get_thinking_json(key));
    const history = parseWasmAny<AutopilotThinking[]>(svc().get_thinking_history_json(key));
    set((s) => ({
      thinking: { ...s.thinking, [key]: thinking },
      thinkingHistory: { ...s.thinkingHistory, [key]: history || [] },
    }));
  },

  setCurrentAutopilotController: (c) => { svc().set_current_controller(c ? JSON.stringify(c) : ""); bump(); },
  removeAutopilotController: (key) => { svc().remove_controller(key); bump(); },
  clearError: () => set({ error: null }),

  getAutopilotControllerByPodKey: (podKey) => {
    const val = parseWasmAny<Ctrl>(svc().get_controller_by_pod_key_json(podKey));
    if (val && ACTIVE.includes(val.phase)) return val;
    return undefined;
  },

  getThinking: (key) => get().thinking[key] || null,
  getThinkingHistory: (key) => get().thinkingHistory[key] || [],
}));

reconnectRegistry.register({
  name: "autopilot:controllers",
  fn: () => useAutopilotStore.getState().fetchAutopilotControllers?.(),
  priority: "low",
});
