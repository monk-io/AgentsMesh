// Connect-RPC adapter for proto.invitation.v1 (InvitationService +
// UserInvitationService + PublicInvitationService).
//
// Encodes requests via @bufbuild/protobuf .toBinary(), passes the Uint8Array
// to the wasm bridge (binary in / binary out per conventions §2.5), decodes
// responses via .fromBinary(). No JSON intermediate.
//
// Returns snake_case web shapes (Invitation, InvitationInfo, PendingInvitation)
// so existing call sites don't have to flip off camelCase + BigInt. Same
// dual-track pattern as billingConnect.ts during the migration window.

import {
  AcceptInvitationRequestSchema,
  AcceptInvitationResponseSchema,
  CreateInvitationRequestSchema,
  GetInvitationByTokenRequestSchema,
  InvitationInfoSchema,
  InvitationSchema,
  ListInvitationsRequestSchema,
  ListInvitationsResponseSchema,
  ListPendingInvitationsRequestSchema,
  ListPendingInvitationsResponseSchema,
  ResendInvitationRequestSchema,
  ResendInvitationResponseSchema,
  RevokeInvitationRequestSchema,
  RevokeInvitationResponseSchema,
  type AcceptedOrgInfo as ProtoAcceptedOrg,
  type Invitation as ProtoInvitation,
  type InvitationInfo as ProtoInvitationInfo,
  type PendingInvitation as ProtoPendingInvitation,
} from "@proto/invitation/v1/invitation_pb";
import { create, toBinary, fromBinary } from "@bufbuild/protobuf";
import { getInvitationService } from "@/lib/wasm-core";
import type {
  Invitation,
  InvitationInfo,
  PendingInvitation,
} from "@/lib/api/invitationTypes";

// ============== Wire conversion (proto -> snake_case web shape) ==============

function fromProtoInvitation(p: ProtoInvitation): Invitation {
  return {
    id: Number(p.id),
    organization_id: Number(p.organizationId),
    email: p.email,
    role: p.role as "admin" | "member",
    expires_at: p.expiresAt,
    accepted_at: p.acceptedAt,
    created_at: p.createdAt,
  };
}

function fromProtoInvitationInfo(p: ProtoInvitationInfo): InvitationInfo {
  return {
    id: Number(p.id),
    email: p.email,
    role: p.role,
    organization_id: Number(p.organizationId),
    organization_name: p.organizationName,
    organization_slug: p.organizationSlug,
    inviter_name: p.inviterName,
    expires_at: p.expiresAt,
    is_expired: p.isExpired,
  };
}

function fromProtoPendingInvitation(p: ProtoPendingInvitation): PendingInvitation {
  return {
    id: Number(p.id),
    organization_id: Number(p.organizationId),
    organization_name: p.organizationName,
    organization_slug: p.organizationSlug,
    role: p.role,
    expires_at: p.expiresAt,
    token: p.token,
  };
}

export interface AcceptedOrgShape {
  id: number;
  name: string;
  slug: string;
}

function fromProtoAcceptedOrg(p: ProtoAcceptedOrg): AcceptedOrgShape {
  return { id: Number(p.id), name: p.name, slug: p.slug };
}

// ============== InvitationService — org-scoped ==============

export async function listInvitations(
  orgSlug: string,
  opts: { offset?: number; limit?: number } = {},
): Promise<{ items: Invitation[]; total: number; limit: number; offset: number }> {
  const req = create(ListInvitationsRequestSchema, {
    orgSlug,
    offset: opts.offset,
    limit: opts.limit,
  });
  const bytes = toBinary(ListInvitationsRequestSchema, req);
  const respBytes = await getInvitationService().listInvitationsConnect(bytes);
  const resp = fromBinary(ListInvitationsResponseSchema, new Uint8Array(respBytes));
  return {
    items: resp.items.map(fromProtoInvitation),
    total: Number(resp.total),
    limit: resp.limit,
    offset: resp.offset,
  };
}

export async function createInvitation(
  orgSlug: string,
  email: string,
  role: string,
): Promise<Invitation> {
  const req = create(CreateInvitationRequestSchema, { orgSlug, email, role });
  const bytes = toBinary(CreateInvitationRequestSchema, req);
  const respBytes = await getInvitationService().createInvitationConnect(bytes);
  return fromProtoInvitation(fromBinary(InvitationSchema, new Uint8Array(respBytes)));
}

export async function revokeInvitation(orgSlug: string, id: number): Promise<void> {
  const req = create(RevokeInvitationRequestSchema, { orgSlug, id: BigInt(id) });
  const bytes = toBinary(RevokeInvitationRequestSchema, req);
  const respBytes = await getInvitationService().revokeInvitationConnect(bytes);
  fromBinary(RevokeInvitationResponseSchema, new Uint8Array(respBytes));
}

export async function resendInvitation(orgSlug: string, id: number): Promise<void> {
  const req = create(ResendInvitationRequestSchema, { orgSlug, id: BigInt(id) });
  const bytes = toBinary(ResendInvitationRequestSchema, req);
  const respBytes = await getInvitationService().resendInvitationConnect(bytes);
  fromBinary(ResendInvitationResponseSchema, new Uint8Array(respBytes));
}

// ============== UserInvitationService — invitee-scoped ==============

export async function acceptInvitation(
  token: string,
): Promise<{ message: string; organization?: AcceptedOrgShape }> {
  const req = create(AcceptInvitationRequestSchema, { token });
  const bytes = toBinary(AcceptInvitationRequestSchema, req);
  const respBytes = await getInvitationService().acceptInvitationConnect(bytes);
  const resp = fromBinary(AcceptInvitationResponseSchema, new Uint8Array(respBytes));
  return {
    message: resp.message,
    organization: resp.organization
      ? fromProtoAcceptedOrg(resp.organization)
      : undefined,
  };
}

export async function listPendingInvitations(): Promise<{
  items: PendingInvitation[];
  total: number;
  limit: number;
  offset: number;
}> {
  const req = create(ListPendingInvitationsRequestSchema, {});
  const bytes = toBinary(ListPendingInvitationsRequestSchema, req);
  const respBytes = await getInvitationService().listPendingInvitationsConnect(bytes);
  const resp = fromBinary(ListPendingInvitationsResponseSchema, new Uint8Array(respBytes));
  return {
    items: resp.items.map(fromProtoPendingInvitation),
    total: Number(resp.total),
    limit: resp.limit,
    offset: resp.offset,
  };
}

// ============== PublicInvitationService — unauthenticated ==============

export async function getInvitationByToken(token: string): Promise<InvitationInfo> {
  const req = create(GetInvitationByTokenRequestSchema, { token });
  const bytes = toBinary(GetInvitationByTokenRequestSchema, req);
  const respBytes = await getInvitationService().getInvitationByTokenConnect(bytes);
  return fromProtoInvitationInfo(fromBinary(InvitationInfoSchema, new Uint8Array(respBytes)));
}
