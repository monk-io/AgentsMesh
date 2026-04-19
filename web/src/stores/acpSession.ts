import { create } from "zustand";
import { getAcpManager } from "@/lib/wasm-core";
import type {
  AcpToolCall, AcpPlanStep, AcpPermissionRequest, AcpSessionState, AcpThinking, AcpLog,
} from "./acpSessionTypes";
import { EMPTY_SESSION, sessionFromWasm, toolCallToWasm, permReqToWasm } from "./acpSessionTypes";

export type { AcpToolCall, AcpPlanStep, AcpPermissionRequest, AcpSessionState, AcpThinking, AcpLog };

type SetFn = (fn: (s: AcpSessionStore) => Partial<AcpSessionStore>) => void;

function syncSession(podKey: string, set: SetFn) {
  const raw = getAcpManager().get_session_json(podKey);
  const session = raw ? sessionFromWasm(typeof raw === "string" ? JSON.parse(raw) : raw) : EMPTY_SESSION;
  set((state) => ({ sessions: { ...state.sessions, [podKey]: session } }));
}

interface AcpSessionStore {
  sessions: Record<string, AcpSessionState>;
  addContentChunk: (podKey: string, sessionId: string, text: string, role: string) => void;
  markLastMessageComplete: (podKey: string) => void;
  updateToolCall: (podKey: string, sessionId: string, toolCall: AcpToolCall) => void;
  setToolCallResult: (podKey: string, sessionId: string, toolCallId: string, success: boolean, resultText: string, errorMessage: string) => void;
  updatePlan: (podKey: string, sessionId: string, steps: AcpPlanStep[]) => void;
  addThinking: (podKey: string, sessionId: string, text: string) => void;
  addPermissionRequest: (podKey: string, req: AcpPermissionRequest) => void;
  removePermissionRequest: (podKey: string, requestId: string) => void;
  updateSessionState: (podKey: string, sessionId: string, state: string) => void;
  addLog: (podKey: string, level: string, message: string) => void;
  clearSession: (podKey: string) => void;
}

export const useAcpSessionStore = create<AcpSessionStore>((set) => ({
  sessions: {},

  addContentChunk: (podKey, _sid, text, role) => {
    getAcpManager().add_content_chunk(podKey, text, role);
    syncSession(podKey, set);
  },

  markLastMessageComplete: (podKey) => {
    getAcpManager().mark_last_message_complete(podKey);
    syncSession(podKey, set);
  },

  updateToolCall: (podKey, _sid, tc) => {
    getAcpManager().update_tool_call(podKey, toolCallToWasm(tc));
    syncSession(podKey, set);
  },

  setToolCallResult: (podKey, _sid, id, success, resultText, errorMessage) => {
    getAcpManager().set_tool_call_result(
      podKey, id, success,
      resultText || undefined,
      errorMessage || undefined,
    );
    syncSession(podKey, set);
  },

  updatePlan: (podKey, _sid, steps) => {
    getAcpManager().update_plan(podKey, JSON.stringify(steps));
    syncSession(podKey, set);
  },

  addThinking: (podKey, _sid, text) => {
    getAcpManager().add_thinking(podKey, text);
    syncSession(podKey, set);
  },

  addPermissionRequest: (podKey, req) => {
    getAcpManager().add_permission_request(podKey, permReqToWasm(req));
    syncSession(podKey, set);
  },

  removePermissionRequest: (podKey, requestId) => {
    getAcpManager().remove_permission_request(podKey, requestId);
    syncSession(podKey, set);
  },

  updateSessionState: (podKey, _sid, state) => {
    getAcpManager().update_session_state(podKey, state);
    syncSession(podKey, set);
  },

  addLog: (podKey, level, message) => {
    getAcpManager().add_log(podKey, level, message);
    syncSession(podKey, set);
  },

  clearSession: (podKey) => {
    getAcpManager().clear_session(podKey);
    set((state) => {
      const { [podKey]: _, ...rest } = state.sessions; void _;
      return { sessions: rest };
    });
  },
}));
