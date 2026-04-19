import { invoke } from "./invoke";

export class ElectronPodState {
  pods_json(): string { return "[]"; }
  current_pod_json(): unknown { return null; }
  get_pod_json(_key: string): unknown { return null; }
  set_pods(json: string) { invoke("podSetPods", json).catch(() => {}); }
  set_current_pod(json: string) { invoke("podSetCurrentPod", json).catch(() => {}); }
  upsert_pod(json: string, ts?: bigint | null) { invoke("podUpsertPod", json, ts ? Number(ts) : null).catch(() => {}); }
  remove_pod(key: string) { invoke("podRemovePod", key).catch(() => {}); }
  update_pod_status(k: string, s: string, as_?: string | null, ec?: string | null, em?: string | null, ts?: bigint | null) {
    invoke("podUpdatePodStatus", k, s, as_ ?? null, ec ?? null, em ?? null, ts ? Number(ts) : null).catch(() => {});
  }
  update_pod_title(k: string, t: string, ts?: bigint | null) { invoke("podUpdatePodTitle", k, t, ts ? Number(ts) : null).catch(() => {}); }
  update_pod_alias(k: string, a: string) { invoke("podUpdatePodAlias", k, a).catch(() => {}); }
  update_agent_status(k: string, s: string) { invoke("podUpdateAgentStatus", k, s).catch(() => {}); }
}

export class ElectronRunnerState {
  runners_json(): string { return "[]"; }
  available_runners_json(): string { return "[]"; }
  current_runner_json(): unknown { return null; }
  get_runner_json(_id: bigint): unknown { return null; }
  set_runners(json: string) { invoke("runnerSetRunners", json).catch(() => {}); }
  set_available_runners(json: string) { invoke("runnerSetAvailableRunners", json).catch(() => {}); }
  set_current_runner(json: string) { invoke("runnerSetCurrentRunner", json).catch(() => {}); }
  update_runner(id: number, json: string) { invoke("runnerUpdateRunnerLocal", id, json).catch(() => {}); }
  update_runner_status(id: number, s: string) { invoke("runnerUpdateRunnerStatus", id, s).catch(() => {}); }
  remove_runner(id: number) { invoke("runnerRemoveRunnerLocal", id).catch(() => {}); }
}

export class ElectronMeshState {
  topology_json(): unknown { return null; }
  selected_node(): unknown { return null; }
  get_node_by_key(_k: string): unknown { return null; }
  get_active_nodes_json(): string { return "[]"; }
  get_nodes_by_runner_json(_id: bigint): string { return "[]"; }
  get_runner_info_json(_id: bigint): unknown { return null; }
  get_edges_for_node_json(_k: string): string { return "[]"; }
  get_channels_for_node_json(_k: string): string { return "[]"; }
  set_topology(json: string) { invoke("meshSetTopology", json).catch(() => {}); }
  clear_topology() { invoke("meshClearTopology").catch(() => {}); }
  select_node(k?: string | null) { invoke("meshSelectNode", k ?? null).catch(() => {}); }
}

export class ElectronTicketState {
  get_tickets(): string { return "[]"; }
  get_ticket_by_slug(_s: string): unknown { return null; }
  get_current_ticket(): unknown { return null; }
  get_board_columns(): string { return "[]"; }
  get_labels(): string { return "[]"; }
}

export class ElectronChannelState {
  channels_json(): string { return "[]"; }
  current_channel_json(): unknown { return null; }
  get_channel_json(_id: bigint): unknown { return null; }
  get_messages_json(_id: bigint): unknown { return null; }
  get_unread_count(_id: bigint): number { return 0; }
  total_unread_count(): number { return 0; }
  unread_counts_json(): string { return "{}"; }
  get_mention_count(_id: bigint): number { return 0; }
  total_mention_count(): number { return 0; }
  mention_counts_json(): string { return "{}"; }
  sorted_channel_ids_json(_m: string, _a: boolean): string { return "[]"; }
  get_last_message_json(_id: bigint): unknown { return null; }
}

export class ElectronLoopState {
  loops_json(): string { return "[]"; }
  current_loop_json(): unknown { return null; }
  runs_json(): string { return "[]"; }
  get_loop_by_slug_json(_s: string): unknown { return null; }
}

export class ElectronAutopilotState {
  controllers_json(): string { return "[]"; }
  current_controller_json(): unknown { return null; }
  get_controller_by_pod_key_json(_k: string): unknown { return null; }
  get_iterations_json(_k: string): string { return "[]"; }
  get_thinking_json(_k: string): unknown { return null; }
  get_thinking_history_json(_k: string): string { return "[]"; }
}

export class ElectronOrgState {
  organizations_json(): string { return "[]"; }
  current_org_json(): unknown { return null; }
  members_json(): string { return "[]"; }
  set_organizations(_json: string) {}
  set_current_org(_json: string) {}
  set_members(_json: string) {}
  add_organization(_json: string) {}
  remove_organization(_id: number) {}
  update_organization(_id: number, _json: string) {}
  add_member(_json: string) {}
  remove_member(_id: string) {}
  update_member(_id: number, _json: string) {}
}

export class ElectronUserState {
  profile_json(): unknown { return null; }
  set_profile(_json: string) {}
  add_identity(_json: string) {}
}

export class ElectronGitProviderState {
  providers_json(): string { return "[]"; }
  current_provider_json(): unknown { return null; }
  available_projects_json(): string { return "[]"; }
  set_providers(_json: string) {}
  set_current_provider(_json: string) {}
  add_provider(_json: string) {}
  update_provider(_id: string, _json: string) {}
  remove_provider(_id: string) {}
  set_available_projects(_json: string) {}
}

export class ElectronRepoState {
  repositories_json(): string { return "[]"; }
  current_repo_json(): unknown { return null; }
  branches_json(): string { return "[]"; }
  set_repositories(_json: string) {}
  set_current_repo(_json: string) {}
  add_repository(_json: string) {}
  update_repository(_id: string, _json: string) {}
  remove_repository(_id: string) {}
  set_branches(_json: string) {}
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
