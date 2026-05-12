// Connect-RPC adapter for proto.promocode.v1 (PromoCodeService).
//
// Encodes requests via @bufbuild/protobuf .toBinary(), passes the Uint8Array
// to the wasm bridge (binary in / binary out per conventions §2.5), decodes
// responses via .fromBinary(). No JSON intermediate.
//
// Returns snake_case web shapes (ValidatePromoCodeResponse,
// RedeemPromoCodeResponse, PromoCodeRedemption) so existing call sites
// don't have to flip off camelCase + BigInt. Same dual-track pattern as
// invitationConnect.ts / billingConnect.ts during the migration window.

import {
  GetRedemptionHistoryRequestSchema,
  GetRedemptionHistoryResponseSchema,
  RedeemPromoCodeRequestSchema,
  RedeemPromoCodeResponseSchema,
  ValidatePromoCodeRequestSchema,
  ValidatePromoCodeResponseSchema,
  type Redemption as ProtoRedemption,
  type RedeemPromoCodeResponse as ProtoRedeemResponse,
  type ValidatePromoCodeResponse as ProtoValidateResponse,
} from "@proto/promocode/v1/promocode_pb";
import { create, toBinary, fromBinary } from "@bufbuild/protobuf";
import { getPromoCodeService } from "@/lib/wasm-core";
import type {
  PromoCodeRedemption,
  RedeemPromoCodeResponse,
  ValidatePromoCodeResponse,
} from "@/lib/api/promoCodeTypes";

// ============== Wire conversion (proto -> snake_case web shape) ==============

export function fromProtoValidateResponse(
  p: ProtoValidateResponse,
): ValidatePromoCodeResponse {
  return {
    valid: p.valid,
    code: p.code,
    plan_name: p.planName,
    plan_display_name: p.planDisplayName,
    duration_months: p.durationMonths,
    expires_at: p.expiresAt,
    message_code: p.messageCode,
  };
}

export function fromProtoRedeemResponse(p: ProtoRedeemResponse): RedeemPromoCodeResponse {
  return {
    success: p.success,
    plan_name: p.planName,
    duration_months: p.durationMonths,
    new_period_end: p.newPeriodEnd,
    message_code: p.messageCode,
  };
}

export function fromProtoRedemption(p: ProtoRedemption): PromoCodeRedemption {
  return {
    id: Number(p.id),
    promo_code_id: Number(p.promoCodeId),
    organization_id: Number(p.organizationId),
    user_id: Number(p.userId),
    plan_name: p.planName,
    duration_months: p.durationMonths,
    previous_plan_name: p.previousPlanName,
    previous_period_end: p.previousPeriodEnd,
    new_period_end: p.newPeriodEnd,
    created_at: p.createdAt,
  };
}

// ============== PromoCodeService — org-scoped ==============

export async function validatePromoCode(
  orgSlug: string,
  code: string,
): Promise<ValidatePromoCodeResponse> {
  const req = create(ValidatePromoCodeRequestSchema, { orgSlug, code });
  const bytes = toBinary(ValidatePromoCodeRequestSchema, req);
  const respBytes = await getPromoCodeService().validatePromoCodeConnect(bytes);
  return fromProtoValidateResponse(
    fromBinary(ValidatePromoCodeResponseSchema, new Uint8Array(respBytes)),
  );
}

export async function redeemPromoCode(
  orgSlug: string,
  code: string,
): Promise<RedeemPromoCodeResponse> {
  const req = create(RedeemPromoCodeRequestSchema, { orgSlug, code });
  const bytes = toBinary(RedeemPromoCodeRequestSchema, req);
  const respBytes = await getPromoCodeService().redeemPromoCodeConnect(bytes);
  return fromProtoRedeemResponse(
    fromBinary(RedeemPromoCodeResponseSchema, new Uint8Array(respBytes)),
  );
}

export async function getRedemptionHistory(
  orgSlug: string,
  opts: { offset?: number; limit?: number } = {},
): Promise<{
  items: PromoCodeRedemption[];
  total: number;
  limit: number;
  offset: number;
}> {
  const req = create(GetRedemptionHistoryRequestSchema, {
    orgSlug,
    offset: opts.offset,
    limit: opts.limit,
  });
  const bytes = toBinary(GetRedemptionHistoryRequestSchema, req);
  const respBytes = await getPromoCodeService().getRedemptionHistoryConnect(bytes);
  const resp = fromBinary(
    GetRedemptionHistoryResponseSchema,
    new Uint8Array(respBytes),
  );
  return {
    items: resp.items.map(fromProtoRedemption),
    total: Number(resp.total),
    limit: resp.limit,
    offset: resp.offset,
  };
}
