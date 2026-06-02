// Re-export hub for Electron state facets that don't share an instance with a
// Service. Pod / Runner / Mesh / Channel / Loop / Autopilot / Repository live
// on their `ElectronXxxService` (the provider aliases `xxxState` to the same
// instance). Ticket has a dedicated state class (./ticket_state) because the
// wasm-side WasmTicketState is no longer co-located with the ticket service.
// The ACP session manager below is still a stub — desktop ACP rendering is
// not wired to a real cache yet.
export { ElectronRelayManager } from "./electron-relay-manager";
export { ElectronTicketState } from "./ticket_state";

export class ElectronAcpManager {
  get_session_json(_podKey: string): unknown { return null; }
  add_content_chunk(_pk: string, _text: string, _role: string) {}
  mark_last_message_complete(_pk: string) {}
  update_tool_call(_req: Uint8Array) {}
  set_tool_call_result(_pk: string, _id: string, _ok: boolean, _r?: string | null, _e?: string | null) {}
  update_plan(_req: Uint8Array) {}
  add_thinking(_pk: string, _text: string) {}
  add_permission_request(_req: Uint8Array) {}
  remove_permission_request(_pk: string, _id: string) {}
  update_session_state(_pk: string, _state: string) {}
  add_log(_pk: string, _level: string, _msg: string) {}
  update_configuration(_req: Uint8Array) {}
  clear_session(_pk: string) {}
}
