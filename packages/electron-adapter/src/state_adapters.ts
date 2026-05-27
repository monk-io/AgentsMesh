// State adapters that can't (yet) share an instance with a Service because
// they have no super-set Service counterpart. Pod / Runner / Mesh /
// Channel / Loop / Autopilot live on the corresponding `ElectronXxxService`
// classes — the provider aliases `xxxState` to the same instance so sync
// getters on the state facet see real cache instead of stub "[]".
// Repository was migrated to this pattern — its state lives on
// `ElectronRepositoryService` now. Ticket has its own dedicated state class
// below because the wasm-side Ticket state is no longer co-located with
// the ticket service (proto-state mutation API + dedicated WasmTicketState).
import type { ITicketState } from "@agentsmesh/service-interface";

// Desktop ticket state stub. Real ticket data flows through Connect-RPC on
// every request — the in-memory cache only exists to keep the renderer's
// store layer from crashing on getTicketState() reads. Future PR will mirror
// the wasm impl (decode proto bytes, maintain a real cache) but until the
// IPC bridge to NAPI grows ticket-state ops, every mutator is a no-op and
// every read returns the empty default.
export class ElectronTicketState implements ITicketState {
  tickets_json(): string { return "[]"; }
  board_columns_json(): string { return "[]"; }
  labels_json(): string { return "[]"; }
  current_ticket_json(): unknown { return null; }

  apply_ticket_status_event(_req: Uint8Array): void {}
  apply_ticket_deleted_event(_req: Uint8Array): void {}
  replace_cached_tickets(_req: Uint8Array): void {}
  insert_created_ticket(_req: Uint8Array): void {}
  patch_cached_ticket(_req: Uint8Array): void {}
  replace_board_columns(_req: Uint8Array): void {}
  append_board_column_tickets(_req: Uint8Array): void {}
  set_current_ticket(_req: Uint8Array): void {}
  replace_cached_labels(_req: Uint8Array): void {}
  insert_created_label(_req: Uint8Array): void {}
  remove_cached_label(_req: Uint8Array): void {}
  filter_tickets(_req: Uint8Array): Uint8Array { return new Uint8Array(); }
}

export class ElectronAcpManager {
  get_session_json(_podKey: string): unknown { return null; }
  add_content_chunk(_pk: string, _text: string, _role: string) {}
  mark_last_message_complete(_pk: string) {}
  update_tool_call(_pk: string, _json: string) {}
  set_tool_call_result(_pk: string, _id: string, _ok: boolean, _r?: string | null, _e?: string | null) {}
  update_plan(_pk: string, _json: string) {}
  add_thinking(_pk: string, _text: string) {}
  add_permission_request(_pk: string, _json: string) {}
  remove_permission_request(_pk: string, _id: string) {}
  update_session_state(_pk: string, _state: string) {}
  add_log(_pk: string, _level: string, _msg: string) {}
  clear_session(_pk: string) {}
}

export class ElectronRelayManager {
  async subscribe(_pk: string, _sid: string, _url: string, _token: string, _cb: Function) {}
  async unsubscribe(_pk: string, _sid: string) {}
  async send(_pk: string, _data: string) {}
  async send_resize(_pk: string, _cols: number, _rows: number) {}
  async force_resize(_pk: string, _cols: number, _rows: number) {}
  async send_acp_command(_pk: string, _cmd: string) {}
  async disconnect(_pk: string) {}
  async disconnect_all() {}
  async get_status(_pk: string): Promise<string> { return "disconnected"; }
  async get_pod_size(_pk: string): Promise<unknown> { return null; }
  async is_runner_disconnected(_pk: string): Promise<boolean> { return true; }
  async on_status_change(_pk: string, _cb: Function) {}
  async on_acp_message(_pk: string, _cb: Function) {}
}
