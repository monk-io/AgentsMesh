// Auth proto-bytes encoders. Renderer view-model (User/Organization with
// snake_case) → proto.auth_state.v1.* request bytes for the AuthManager
// mutator surface. Kept out of stores/auth.ts so that file stays under the
// 200-line SRP cap.

import { create as protoCreate, toBinary } from "@bufbuild/protobuf";
import {
  ApplySessionRequestSchema,
  SetCurrentOrgRequestSchema,
  SetOrganizationsRequestSchema,
} from "@proto/auth_state/v1/auth_state_pb";

export interface AuthUserVM {
  id: number;
  email: string;
  username: string;
  name?: string;
  avatar_url?: string;
}

export interface AuthOrgVM {
  id: number;
  name: string;
  slug: string;
  role?: string;
  logo_url?: string;
  subscription_plan?: string;
  subscription_status?: string;
  created_at?: string;
  updated_at?: string;
}

function userToProto(u: AuthUserVM) {
  return {
    id: BigInt(u.id),
    email: u.email,
    username: u.username,
    name: u.name,
    avatarUrl: u.avatar_url,
  };
}

function orgToProto(o: AuthOrgVM) {
  return {
    id: BigInt(o.id),
    name: o.name,
    slug: o.slug,
    logoUrl: o.logo_url,
    subscriptionPlan: o.subscription_plan ?? "",
    subscriptionStatus: o.subscription_status ?? "",
    role: o.role,
    createdAt: o.created_at ?? "",
    updatedAt: o.updated_at ?? "",
  };
}

export function encodeApplySession(
  token: string,
  user: AuthUserVM,
  refreshToken?: string,
): Uint8Array {
  const req = protoCreate(ApplySessionRequestSchema, {
    token,
    refreshToken: refreshToken || "",
    user: userToProto(user),
  });
  return toBinary(ApplySessionRequestSchema, req);
}

export function encodeSetOrganizations(orgs: AuthOrgVM[]): Uint8Array {
  const req = protoCreate(SetOrganizationsRequestSchema, {
    items: orgs.map(orgToProto),
  });
  return toBinary(SetOrganizationsRequestSchema, req);
}

export function encodeSetCurrentOrg(org: AuthOrgVM): Uint8Array {
  const req = protoCreate(SetCurrentOrgRequestSchema, {
    org: orgToProto(org),
  });
  return toBinary(SetCurrentOrgRequestSchema, req);
}
