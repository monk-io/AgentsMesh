// Unit tests for clients/web/src/lib/api/promocodeConnect.ts proto →
// snake_case converters. Pins the field-by-field translation across the
// binary wire so the wire-drift bug class (PR 986a38ca6) is compile-time
// impossible for promocode.
//
// Note: this test file imports @proto/promocode/v1/promocode_pb which
// resolves via vitest.config.ts. Under `bazel test //clients/web:unit` the
// `proto/gen` tree is in .bazelignore so the import fails — runs under
// `pnpm test:run` (or once ts_proto_library lands; see runbook
// "TS proto codegen toolchain").

import { describe, it, expect } from "vitest";
import { create } from "@bufbuild/protobuf";
import {
  RedeemPromoCodeResponseSchema,
  RedemptionSchema,
  ValidatePromoCodeResponseSchema,
} from "@proto/promocode/v1/promocode_pb";
import {
  fromProtoRedeemResponse,
  fromProtoRedemption,
  fromProtoValidateResponse,
} from "../promocodeConnect";

describe("fromProtoValidateResponse", () => {
  it("converts the valid path with all fields", () => {
    const proto = create(ValidatePromoCodeResponseSchema, {
      valid: true,
      code: "WELCOME2026",
      planName: "pro",
      planDisplayName: "Pro",
      durationMonths: 3,
      expiresAt: "2026-12-31T23:59:59Z",
    });
    const data = fromProtoValidateResponse(proto);
    expect(data).toEqual({
      valid: true,
      code: "WELCOME2026",
      plan_name: "pro",
      plan_display_name: "Pro",
      duration_months: 3,
      expires_at: "2026-12-31T23:59:59Z",
      message_code: undefined,
    });
  });

  it("converts the invalid path with message_code", () => {
    const proto = create(ValidatePromoCodeResponseSchema, {
      valid: false,
      code: "BOGUS",
      messageCode: "promo_code_not_found",
    });
    const data = fromProtoValidateResponse(proto);
    expect(data.valid).toBe(false);
    expect(data.message_code).toBe("promo_code_not_found");
    expect(data.plan_name).toBeUndefined();
    expect(data.duration_months).toBeUndefined();
  });
});

describe("fromProtoRedeemResponse", () => {
  it("converts a success response", () => {
    const proto = create(RedeemPromoCodeResponseSchema, {
      success: true,
      planName: "pro",
      durationMonths: 3,
      newPeriodEnd: "2026-08-01T00:00:00Z",
      messageCode: "promo_code_redeem_success",
    });
    const data = fromProtoRedeemResponse(proto);
    expect(data).toEqual({
      success: true,
      plan_name: "pro",
      duration_months: 3,
      new_period_end: "2026-08-01T00:00:00Z",
      message_code: "promo_code_redeem_success",
    });
  });

  it("converts a non-owner failure response", () => {
    const proto = create(RedeemPromoCodeResponseSchema, {
      success: false,
      messageCode: "promo_code_not_owner",
    });
    const data = fromProtoRedeemResponse(proto);
    expect(data.success).toBe(false);
    expect(data.message_code).toBe("promo_code_not_owner");
    expect(data.plan_name).toBeUndefined();
    expect(data.new_period_end).toBeUndefined();
  });
});

describe("fromProtoRedemption", () => {
  it("converts a full Redemption proto preserving every field", () => {
    const proto = create(RedemptionSchema, {
      id: BigInt(1),
      promoCodeId: BigInt(7),
      organizationId: BigInt(42),
      userId: BigInt(100),
      planName: "pro",
      durationMonths: 3,
      previousPlanName: "free",
      previousPeriodEnd: "2026-05-01T00:00:00Z",
      newPeriodEnd: "2026-08-01T00:00:00Z",
      ipAddress: "203.0.113.42",
      userAgent: "Mozilla/5.0",
      createdAt: "2026-05-12T13:16:10Z",
    });
    const data = fromProtoRedemption(proto);
    expect(data).toEqual({
      id: 1,
      promo_code_id: 7,
      organization_id: 42,
      user_id: 100,
      plan_name: "pro",
      duration_months: 3,
      previous_plan_name: "free",
      previous_period_end: "2026-05-01T00:00:00Z",
      new_period_end: "2026-08-01T00:00:00Z",
      created_at: "2026-05-12T13:16:10Z",
    });
  });

  it("narrows BigInt to number for id / promo_code_id / organization_id / user_id", () => {
    const proto = create(RedemptionSchema, {
      id: BigInt(2_000_000),
      promoCodeId: BigInt(3_000_000),
      organizationId: BigInt(4_000_000),
      userId: BigInt(5_000_000),
      planName: "x",
      durationMonths: 1,
      newPeriodEnd: "2026-06-01T00:00:00Z",
      createdAt: "2026-05-12T00:00:00Z",
    });
    const data = fromProtoRedemption(proto);
    expect(typeof data.id).toBe("number");
    expect(typeof data.promo_code_id).toBe("number");
    expect(typeof data.organization_id).toBe("number");
    expect(typeof data.user_id).toBe("number");
    expect(data.id).toBe(2_000_000);
    expect(data.user_id).toBe(5_000_000);
  });

  it("treats absent optional fields as undefined", () => {
    const proto = create(RedemptionSchema, {
      id: BigInt(1),
      promoCodeId: BigInt(7),
      organizationId: BigInt(42),
      userId: BigInt(100),
      planName: "enterprise",
      durationMonths: 12,
      newPeriodEnd: "2027-05-01T00:00:00Z",
      createdAt: "2026-05-12T13:16:10Z",
    });
    const data = fromProtoRedemption(proto);
    expect(data.previous_plan_name).toBeUndefined();
    expect(data.previous_period_end).toBeUndefined();
  });
});
