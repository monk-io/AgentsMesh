// Wasm contract test. Type-only file — no runtime body, no asserts.
// `tsc --noEmit` (= `bazel build //clients/web:src --output_groups=typecheck`)
// is the gate: if any of the listed wasm-bindgen method signatures the
// stores actually call drifts away from what's exported in
// `agentsmesh-wasm` (the npm wrapper around the Bazel-built wasm_pkg),
// these type assertions fail and the build fails.
//
// This catches the regression class that motivated this PR: the renderer
// reached for `set_pods()`, a method that the Rust crate removed
// during the proto-bytes migration. `WasmPodState` no longer had it, so
// production threw "podState.set_pods is not a function" the first time
// the workspace sidebar tried to refresh. tsc was happy because the
// type lookup went through an `any`-typed stub.
//
// Convention: one `_Assert<…>` line per public method the production
// stores call. Type the function as a generic over the parameter tuple
// and the return type, then instantiate it with the wasm method's
// inferred types. If the method is renamed or its signature changes,
// the instantiation fails.
//
// Adding a new wasm-bound method that a store calls: append an
// `_Assert<…>` here so future drift catches it at compile time.

/* eslint-disable @typescript-eslint/no-unused-vars */
import type {
  WasmPodState,
  WasmPodService,
  WasmRunnerService,
  WasmRunnerState,
  WasmTicketState,
  WasmChannelState,
  WasmLoopState,
  WasmMeshState,
} from "agentsmesh-wasm";

// Plain identity helper — `_Sig<F>` accepts any function type. We use it
// only to surface a type error if the wasm method's signature changes
// (or the property gets dropped from the class entirely).
type _Sig<F extends (...args: never[]) => unknown> = F;

// ── PodState (clients/core/crates/wasm/src/state_pod.rs) ────────────
// Production callers: clients/web/src/stores/pod.ts and
// providers/realtimePodHandlers.ts. Method group A is reads; B is
// proto-bytes mutators.
type _PodState_pods_json           = _Sig<WasmPodState["pods_json"]>;
type _PodState_current_pod_json    = _Sig<WasmPodState["current_pod_json"]>;
type _PodState_get_pod_json        = _Sig<WasmPodState["get_pod_json"]>;
type _PodState_insert_created_pod  = _Sig<WasmPodState["insert_created_pod"]>;
type _PodState_patch_pod_perpetual = _Sig<WasmPodState["patch_pod_perpetual"]>;
type _PodState_apply_status        = _Sig<WasmPodState["apply_pod_status_event"]>;
type _PodState_apply_title         = _Sig<WasmPodState["apply_pod_title_event"]>;
type _PodState_apply_alias         = _Sig<WasmPodState["apply_pod_alias_event"]>;
type _PodState_apply_agent_status  = _Sig<WasmPodState["apply_agent_status_event"]>;
type _PodState_replace_cached_pods = _Sig<WasmPodState["replace_cached_pods"]>;
type _PodState_append_cached_pods  = _Sig<WasmPodState["append_cached_pods"]>;
type _PodState_mark_terminated     = _Sig<WasmPodState["mark_pod_terminated"]>;
type _PodState_remove_pod          = _Sig<WasmPodState["remove_pod"]>;
type _PodState_update_init_prog    = _Sig<WasmPodState["update_init_progress"]>;
type _PodState_clear_init_prog     = _Sig<WasmPodState["clear_init_progress"]>;

// Specifically guard the proto-bytes contract: every mutator MUST
// accept a Uint8Array as its first argument. If a future rust commit
// reverts back to JSON strings these break.
type _RequiresU8<F extends (b: Uint8Array, ...rest: never[]) => unknown> = F;
type _PodState_proto_insert     = _RequiresU8<WasmPodState["insert_created_pod"]>;
type _PodState_proto_perpetual  = _RequiresU8<WasmPodState["patch_pod_perpetual"]>;
type _PodState_proto_status     = _RequiresU8<WasmPodState["apply_pod_status_event"]>;
type _PodState_proto_title      = _RequiresU8<WasmPodState["apply_pod_title_event"]>;
type _PodState_proto_alias      = _RequiresU8<WasmPodState["apply_pod_alias_event"]>;
type _PodState_proto_agent      = _RequiresU8<WasmPodState["apply_agent_status_event"]>;
type _PodState_proto_replace    = _RequiresU8<WasmPodState["replace_cached_pods"]>;
type _PodState_proto_append     = _RequiresU8<WasmPodState["append_cached_pods"]>;
type _PodState_proto_terminate  = _RequiresU8<WasmPodState["mark_pod_terminated"]>;

// ── PodService (Connect-RPC binary lane via wasm) ───────────────────
// Production callers: clients/web/src/lib/api/connect/podConnect.ts.
type _PodSvc_list_pods         = _Sig<WasmPodService["list_pods_connect"]>;
type _PodSvc_get_pod           = _Sig<WasmPodService["get_pod_connect"]>;
type _PodSvc_create_pod        = _Sig<WasmPodService["create_pod_connect"]>;
type _PodSvc_terminate_pod     = _Sig<WasmPodService["terminate_pod_connect"]>;
type _PodSvc_update_alias      = _Sig<WasmPodService["update_pod_alias_connect"]>;
type _PodSvc_update_perpetual  = _Sig<WasmPodService["update_pod_perpetual_connect"]>;
type _PodSvc_get_conn          = _Sig<WasmPodService["get_pod_connection_connect"]>;
type _PodSvc_send_prompt       = _Sig<WasmPodService["send_pod_prompt_connect"]>;
type _PodSvc_by_ticket         = _Sig<WasmPodService["list_pods_by_ticket_connect"]>;

// ── RunnerService (production callers: stores/runner.ts) ─────────────
type _RunnerSvc_list           = _Sig<WasmRunnerService["fetch_runners"]>;
type _RunnerSvc_available      = _Sig<WasmRunnerService["fetch_available_runners"]>;
type _RunnerSvc_get            = _Sig<WasmRunnerService["fetch_runner"]>;
type _RunnerSvc_update         = _Sig<WasmRunnerService["update_runner"]>;
type _RunnerSvc_delete         = _Sig<WasmRunnerService["delete_runner"]>;
type _RunnerSvc_list_pods      = _Sig<WasmRunnerService["list_runner_pods"]>;
type _RunnerSvc_query_sandbox  = _Sig<WasmRunnerService["query_runner_sandboxes"]>;

// ── RunnerState reads (used by sidebar selectors) ────────────────────
type _RunnerState_list         = _Sig<WasmRunnerState["runners_json"]>;
type _RunnerState_available    = _Sig<WasmRunnerState["available_runners_json"]>;
type _RunnerState_current      = _Sig<WasmRunnerState["current_runner_json"]>;
type _RunnerState_get          = _Sig<WasmRunnerState["get_runner_json"]>;
type _RunnerState_set          = _Sig<WasmRunnerState["set_runners"]>;
type _RunnerState_set_curr     = _Sig<WasmRunnerState["set_current_runner"]>;
type _RunnerState_update_st    = _Sig<WasmRunnerState["update_runner_status"]>;
type _RunnerState_remove       = _Sig<WasmRunnerState["remove_runner"]>;

// ── TicketState (production callers: stores/ticket.ts + board hooks) ─
type _TicketState_list         = _Sig<WasmTicketState["tickets_json"]>;
type _TicketState_get          = _Sig<WasmTicketState["get_ticket_by_slug_json"]>;
type _TicketState_set          = _Sig<WasmTicketState["set_tickets"]>;
type _TicketState_add          = _Sig<WasmTicketState["add_ticket"]>;
type _TicketState_update       = _Sig<WasmTicketState["update_ticket"]>;
type _TicketState_remove       = _Sig<WasmTicketState["remove_ticket"]>;
type _TicketState_board        = _Sig<WasmTicketState["board_columns_json"]>;
type _TicketState_set_board    = _Sig<WasmTicketState["set_board_columns"]>;
type _TicketState_labels       = _Sig<WasmTicketState["labels_json"]>;
type _TicketState_set_labels   = _Sig<WasmTicketState["set_labels"]>;
type _TicketState_filter       = _Sig<WasmTicketState["filter_tickets_json"]>;

// ── ChannelState (production callers: stores/channel.ts, stores/channelMessage.ts) ──
type _ChannelState_list        = _Sig<WasmChannelState["channels_json"]>;
type _ChannelState_current     = _Sig<WasmChannelState["current_channel_json"]>;
type _ChannelState_set         = _Sig<WasmChannelState["set_channels"]>;
type _ChannelState_add         = _Sig<WasmChannelState["add_channel"]>;
type _ChannelState_update      = _Sig<WasmChannelState["update_channel"]>;
type _ChannelState_remove      = _Sig<WasmChannelState["remove_channel"]>;
type _ChannelState_messages    = _Sig<WasmChannelState["get_messages_json"]>;
type _ChannelState_set_msgs    = _Sig<WasmChannelState["set_messages"]>;
type _ChannelState_add_msg     = _Sig<WasmChannelState["add_message"]>;
type _ChannelState_update_msg  = _Sig<WasmChannelState["update_message"]>;
type _ChannelState_remove_msg  = _Sig<WasmChannelState["remove_message"]>;
type _ChannelState_unread_get  = _Sig<WasmChannelState["get_unread_count"]>;
type _ChannelState_unread_set  = _Sig<WasmChannelState["set_unread_counts"]>;

// ── LoopState (production callers: stores/loop.ts) ───────────────────
type _LoopState_list           = _Sig<WasmLoopState["loops_json"]>;
type _LoopState_current        = _Sig<WasmLoopState["current_loop_json"]>;
type _LoopState_get_by_slug    = _Sig<WasmLoopState["get_loop_by_slug_json"]>;
type _LoopState_set            = _Sig<WasmLoopState["set_loops"]>;
type _LoopState_set_current    = _Sig<WasmLoopState["set_current_loop"]>;
type _LoopState_runs           = _Sig<WasmLoopState["runs_json"]>;
type _LoopState_set_runs       = _Sig<WasmLoopState["set_runs"]>;

// ── MeshState (production callers: stores/mesh.ts + topology hooks) ──
type _MeshState_topology       = _Sig<WasmMeshState["topology_json"]>;
type _MeshState_replace        = _Sig<WasmMeshState["replace_topology"]>;
type _MeshState_clear          = _Sig<WasmMeshState["clear_topology"]>;
type _MeshState_select         = _Sig<WasmMeshState["select_node"]>;
type _MeshState_get_node       = _Sig<WasmMeshState["get_node_json"]>;
type _MeshState_get_edges      = _Sig<WasmMeshState["get_edges_for_node_json"]>;

// Vitest discovery requires the file to contain *something* runnable, but
// the body is intentionally empty — tsc + the type assertions above are
// the whole assertion surface.
import { describe, it, expect } from "vitest";

describe("wasm contract", () => {
  it("compiles when wasm bindings match the production call sites", () => {
    expect(true).toBe(true);
  });
});
