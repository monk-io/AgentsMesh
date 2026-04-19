import { getInvitationService } from "@/lib/wasm-core";
export type { Invitation, InvitationInfo, PendingInvitation } from "./invitationTypes";

export const invitationApi = {
  getByToken: async (token: string) => {
    const json = await getInvitationService().get_by_token(token);
    return JSON.parse(json);
  },
  accept: async (token: string) => {
    await getInvitationService().accept(token);
  },
  list: async () => {
    const json = await getInvitationService().list();
    return JSON.parse(json);
  },
  create: async (email: string, role: string) => {
    const json = await getInvitationService().create(JSON.stringify({ email, role }));
    return JSON.parse(json);
  },
  revoke: async (id: number) => {
    await getInvitationService().revoke(BigInt(id));
  },
  resend: async (id: number) => {
    await getInvitationService().resend(BigInt(id));
  },
};
