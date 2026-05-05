import { invoke } from "./invoke";
import type { IInvitationService } from "@agentsmesh/service-interface";

export class ElectronInvitationService implements IInvitationService {
  async list(): Promise<string> {
    return invoke<string>("invitationList");
  }

  async list_pending(): Promise<string> {
    return invoke<string>("invitationListPending");
  }

  async create(json: string): Promise<string> {
    return invoke<string>("invitationCreate", json);
  }

  async get_by_token(token: string): Promise<string> {
    return invoke<string>("invitationGetByToken", token);
  }

  async accept(token: string): Promise<void> {
    await invoke<void>("invitationAccept", token);
  }

  async revoke(id: bigint): Promise<void> {
    await invoke<void>("invitationRevoke", Number(id));
  }

  async resend(id: bigint): Promise<void> {
    await invoke<void>("invitationResend", Number(id));
  }
}
