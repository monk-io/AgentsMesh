import { invoke } from "./invoke";
import type { IBindingService } from "@agentsmesh/service-interface";

export class ElectronBindingService implements IBindingService {
  async list_bindings(status?: string | null): Promise<string> {
    return invoke<string>("bindingListBindings", status);
  }

  async get_bound_pods(): Promise<string> {
    return invoke<string>("bindingGetBoundPods");
  }

  async get_pending_bindings(): Promise<string> {
    return invoke<string>("bindingGetPendingBindings");
  }

  async check_binding(targetPod: string): Promise<string> {
    return invoke<string>("bindingCheckBinding", targetPod);
  }

  async request_binding(json: string, podKey?: string | null): Promise<string> {
    return invoke<string>("bindingRequestBinding", json, podKey);
  }

  async accept_binding(json: string): Promise<string> {
    return invoke<string>("bindingAcceptBinding", json);
  }

  async reject_binding(json: string): Promise<void> {
    await invoke<void>("bindingRejectBinding", json);
  }

  async unbind(json: string): Promise<void> {
    await invoke<void>("bindingUnbind", json);
  }

  async request_scopes(bindingId: bigint, json: string): Promise<string> {
    return invoke<string>("bindingRequestScopes", Number(bindingId), json);
  }

  async approve_scopes(bindingId: bigint, json: string): Promise<string> {
    return invoke<string>("bindingApproveScopes", Number(bindingId), json);
  }
}
