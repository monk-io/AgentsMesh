// Invitation fetch + accept over Connect-RPC JSON. GetInvitationByToken is
// public (no auth — the token IS the auth); AcceptInvitation requires the
// invitee's JWT so the auth interceptor can match the token's email.

import { lightConnect } from "./api-fetch";
import { updateLightSessionOrgSlug } from "@/lib/light-session";
import type { InvitationInfo } from "@/lib/api/invitationTypes";

interface ConnectInvitationInfo {
  id: number | string;
  email: string;
  role: string;
  organizationId: number | string;
  organizationName: string;
  organizationSlug: string;
  inviterName: string;
  expiresAt: string;
  isExpired: boolean;
}

interface GetByTokenResponse {
  invitation?: ConnectInvitationInfo;
}

function toInfo(i: ConnectInvitationInfo): InvitationInfo {
  return {
    id: Number(i.id),
    email: i.email,
    role: i.role,
    organization_id: Number(i.organizationId),
    organization_name: i.organizationName,
    organization_slug: i.organizationSlug,
    inviter_name: i.inviterName,
    expires_at: i.expiresAt,
    is_expired: i.isExpired,
  };
}

export async function lightFetchInvitation(token: string): Promise<InvitationInfo | null> {
  const resp = await lightConnect<{ token: string }, GetByTokenResponse>(
    "proto.invitation.v1.PublicInvitationService",
    "GetInvitationByToken",
    { token },
  );
  return resp?.invitation ? toInfo(resp.invitation) : null;
}

export async function lightAcceptInvitation(
  token: string,
  organizationSlug: string,
): Promise<void> {
  await lightConnect<{ token: string }, unknown>(
    "proto.invitation.v1.UserInvitationService",
    "AcceptInvitation",
    { token },
    { authenticated: true },
  );
  // Make the just-joined org the current one so the post-accept redirect
  // lands on its workspace. Dashboard bootstrap may overwrite this once
  // wasm reads /users/me/organizations, but the eager update prevents a
  // flicker / wrong-org render during the transition.
  updateLightSessionOrgSlug(organizationSlug);
}
