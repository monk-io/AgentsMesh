// Connect-RPC adapter for proto.billing.v1.BillingService + BillingPublicService.
//
// Encodes requests via @bufbuild/protobuf .toBinary(), passes the Uint8Array
// to the wasm bridge (binary in / binary out per conventions §2.5), decodes
// responses via .fromBinary(). No JSON intermediate.
//
// Returns snake_case web shapes (Subscription, BillingOverview, etc.) so call
// sites don't have to switch wire-camelCase off the proto generated types.
// PR #334 fix preserved: public pricing card retains price_monthly /
// price_yearly / currency end-to-end through the binary wire.

import {
  BillingOverviewSchema,
  CancelSubscriptionRequestSchema,
  CancelSubscriptionResponseSchema,
  ChangeBillingCycleRequestSchema,
  ChangeBillingCycleResponseSchema,
  CheckoutStatusSchema,
  CreateCheckoutRequestSchema,
  CreateCheckoutResponseSchema,
  CreateSubscriptionRequestSchema,
  DeploymentInfoSchema,
  GetCheckoutStatusRequestSchema,
  GetDeploymentInfoRequestSchema,
  GetOverviewRequestSchema,
  GetPublicDeploymentInfoRequestSchema,
  GetPublicPricingRequestSchema,
  GetSeatUsageRequestSchema,
  GetSubscriptionRequestSchema,
  ListInvoicesRequestSchema,
  ListInvoicesResponseSchema,
  ListPlansRequestSchema,
  ListPlansResponseSchema,
  PublicPricingResponseSchema,
  PurchaseSeatsRequestSchema,
  PurchaseSeatsResponseSchema,
  ReactivateSubscriptionRequestSchema,
  RequestCancelSubscriptionRequestSchema,
  RequestCancelSubscriptionResponseSchema,
  SeatUsageSchema,
  SubscriptionSchema,
  UpdateAutoRenewRequestSchema,
  UpdateSubscriptionRequestSchema,
  UpgradeSubscriptionRequestSchema,
  type BillingOverview as ProtoBillingOverview,
  type CheckoutStatus as ProtoCheckoutStatus,
  type DeploymentInfo as ProtoDeploymentInfo,
  type Invoice as ProtoInvoice,
  type PublicPlanPricing as ProtoPublicPlanPricing,
  type PublicPricingResponse as ProtoPublicPricingResponse,
  type SeatUsage as ProtoSeatUsage,
  type Subscription as ProtoSubscription,
  type SubscriptionPlan as ProtoSubscriptionPlan,
} from "@proto/billing/v1/billing_pb";
import { create, toBinary, fromBinary } from "@bufbuild/protobuf";
import { getBillingService } from "@/lib/wasm-core";
import type {
  BillingCycle,
  BillingOverview,
  CheckoutResponse,
  CheckoutStatus,
  Currency,
  DeploymentInfo,
  Invoice,
  OrderType,
  PaymentProvider,
  PublicPlanPricing,
  PublicPricingResponse,
  SeatUsage,
  Subscription,
  SubscriptionPlan,
} from "@/lib/api/billing-types";

// ============== Wire conversion (proto -> snake_case web shape) ==============

function fromProtoPlan(p: ProtoSubscriptionPlan): SubscriptionPlan {
  return {
    id: Number(p.id),
    name: p.name,
    display_name: p.displayName,
    price_per_seat_monthly: p.pricePerSeatMonthly,
    price_per_seat_yearly: p.pricePerSeatYearly,
    included_pod_minutes: p.includedPodMinutes,
    price_per_extra_minute: p.pricePerExtraMinute,
    max_users: p.maxUsers,
    max_runners: p.maxRunners,
    max_repositories: p.maxRepositories,
    max_concurrent_pods: p.maxConcurrentPods,
    features: {},
    is_active: p.isActive,
  };
}

function fromProtoSubscription(s: ProtoSubscription): Subscription {
  return {
    id: Number(s.id),
    organization_id: Number(s.organizationId),
    plan_id: Number(s.planId),
    status: s.status,
    billing_cycle: s.billingCycle,
    current_period_start: s.currentPeriodStart,
    current_period_end: s.currentPeriodEnd,
    plan: s.plan ? fromProtoPlan(s.plan) : undefined,
    payment_provider: s.paymentProvider,
    payment_method: s.paymentMethod,
    auto_renew: s.autoRenew,
    cancel_at_period_end: s.cancelAtPeriodEnd,
    seat_count: s.seatCount,
    stripe_customer_id: s.stripeCustomerId,
    stripe_subscription_id: s.stripeSubscriptionId,
    lemonsqueezy_customer_id: s.lemonsqueezyCustomerId,
    lemonsqueezy_subscription_id: s.lemonsqueezySubscriptionId,
    frozen_at: s.frozenAt,
    downgrade_to_plan: s.downgradeToPlan,
    next_billing_cycle: s.nextBillingCycle,
  };
}

function fromProtoOverview(o: ProtoBillingOverview): BillingOverview {
  return {
    plan: o.plan ? fromProtoPlan(o.plan) : ({} as SubscriptionPlan),
    status: o.status,
    billing_cycle: o.billingCycle,
    current_period_start: o.currentPeriodStart,
    current_period_end: o.currentPeriodEnd,
    cancel_at_period_end: o.cancelAtPeriodEnd,
    usage: o.usage
      ? {
          pod_minutes: o.usage.podMinutes,
          included_pod_minutes: o.usage.includedPodMinutes,
          users: o.usage.users,
          max_users: o.usage.maxUsers,
          runners: o.usage.runners,
          max_runners: o.usage.maxRunners,
          repositories: o.usage.repositories,
          max_repositories: o.usage.maxRepositories,
        }
      : ({} as BillingOverview["usage"]),
  };
}

function fromProtoSeatUsage(u: ProtoSeatUsage): SeatUsage {
  return {
    total_seats: u.totalSeats,
    used_seats: u.usedSeats,
    available_seats: u.availableSeats,
    max_seats: u.maxSeats,
    can_add_seats: u.canAddSeats,
  };
}

function fromProtoDeployment(d: ProtoDeploymentInfo): DeploymentInfo {
  return {
    deployment_type: (d.deploymentType || "global") as DeploymentInfo["deployment_type"],
    available_providers: d.availableProviders,
  };
}

function fromProtoInvoice(i: ProtoInvoice): Invoice {
  return {
    id: Number(i.id),
    organization_id: Number(i.organizationId),
    invoice_no: i.invoiceNo,
    order_no: undefined,
    amount: i.subtotal,
    tax_amount: i.taxAmount,
    total_amount: i.total,
    currency: i.currency,
    status: i.status,
    billing_period_start: i.periodStart,
    billing_period_end: i.periodEnd,
    paid_at: i.paidAt,
    created_at: i.createdAt,
  };
}

function fromProtoCheckoutStatus(c: ProtoCheckoutStatus): CheckoutStatus {
  return {
    order_no: c.orderNo,
    status: c.status,
    order_type: c.orderType,
    amount: c.amount,
    currency: c.currency,
    created_at: c.createdAt,
    paid_at: c.paidAt,
  };
}

function fromProtoPublicPlan(p: ProtoPublicPlanPricing): PublicPlanPricing {
  return {
    name: p.name,
    display_name: p.displayName,
    price_monthly: p.priceMonthly,
    price_yearly: p.priceYearly,
    max_users: p.maxUsers,
    max_runners: p.maxRunners,
    max_repositories: p.maxRepositories,
    max_concurrent_pods: p.maxConcurrentPods,
  };
}

function fromProtoPublicPricing(r: ProtoPublicPricingResponse): PublicPricingResponse {
  return {
    deployment_type: r.deploymentType,
    currency: (r.currency || "USD") as Currency,
    plans: r.plans.map(fromProtoPublicPlan),
  };
}

// ============== BillingService — auth-required, org-scoped ==============

export async function getOverviewConnect(orgSlug: string): Promise<BillingOverview> {
  const req = create(GetOverviewRequestSchema, { orgSlug });
  const bytes = toBinary(GetOverviewRequestSchema, req);
  const respBytes = await getBillingService().get_overview_connect(bytes);
  return fromProtoOverview(fromBinary(BillingOverviewSchema, new Uint8Array(respBytes)));
}

export async function listPlansConnect(orgSlug: string): Promise<SubscriptionPlan[]> {
  const req = create(ListPlansRequestSchema, { orgSlug });
  const bytes = toBinary(ListPlansRequestSchema, req);
  const respBytes = await getBillingService().list_plans_connect(bytes);
  const resp = fromBinary(ListPlansResponseSchema, new Uint8Array(respBytes));
  return resp.items.map(fromProtoPlan);
}

export async function getSubscriptionConnect(orgSlug: string): Promise<Subscription> {
  const req = create(GetSubscriptionRequestSchema, { orgSlug });
  const bytes = toBinary(GetSubscriptionRequestSchema, req);
  const respBytes = await getBillingService().get_subscription_connect(bytes);
  return fromProtoSubscription(fromBinary(SubscriptionSchema, new Uint8Array(respBytes)));
}

export async function createSubscriptionConnect(
  orgSlug: string,
  planName: string,
  billingCycle?: BillingCycle,
): Promise<Subscription> {
  const req = create(CreateSubscriptionRequestSchema, { orgSlug, planName, billingCycle });
  const bytes = toBinary(CreateSubscriptionRequestSchema, req);
  const respBytes = await getBillingService().create_subscription_connect(bytes);
  return fromProtoSubscription(fromBinary(SubscriptionSchema, new Uint8Array(respBytes)));
}

export async function updateSubscriptionConnect(
  orgSlug: string,
  planName: string,
): Promise<Subscription> {
  const req = create(UpdateSubscriptionRequestSchema, { orgSlug, planName });
  const bytes = toBinary(UpdateSubscriptionRequestSchema, req);
  const respBytes = await getBillingService().update_subscription_connect(bytes);
  return fromProtoSubscription(fromBinary(SubscriptionSchema, new Uint8Array(respBytes)));
}

export async function cancelSubscriptionConnect(orgSlug: string): Promise<void> {
  const req = create(CancelSubscriptionRequestSchema, { orgSlug });
  const bytes = toBinary(CancelSubscriptionRequestSchema, req);
  const respBytes = await getBillingService().cancel_subscription_connect(bytes);
  fromBinary(CancelSubscriptionResponseSchema, new Uint8Array(respBytes));
}

export async function requestCancelSubscriptionConnect(
  orgSlug: string,
  immediate: boolean,
): Promise<{ immediate: boolean; current_period_end?: string }> {
  const req = create(RequestCancelSubscriptionRequestSchema, { orgSlug, immediate });
  const bytes = toBinary(RequestCancelSubscriptionRequestSchema, req);
  const respBytes = await getBillingService().request_cancel_connect(bytes);
  const resp = fromBinary(RequestCancelSubscriptionResponseSchema, new Uint8Array(respBytes));
  return { immediate: resp.immediate, current_period_end: resp.currentPeriodEnd };
}

export async function reactivateSubscriptionConnect(orgSlug: string): Promise<Subscription> {
  const req = create(ReactivateSubscriptionRequestSchema, { orgSlug });
  const bytes = toBinary(ReactivateSubscriptionRequestSchema, req);
  const respBytes = await getBillingService().reactivate_connect(bytes);
  return fromProtoSubscription(fromBinary(SubscriptionSchema, new Uint8Array(respBytes)));
}

export async function upgradeSubscriptionConnect(
  orgSlug: string,
  planName: string,
): Promise<Subscription> {
  const req = create(UpgradeSubscriptionRequestSchema, { orgSlug, planName });
  const bytes = toBinary(UpgradeSubscriptionRequestSchema, req);
  const respBytes = await getBillingService().upgrade_connect(bytes);
  return fromProtoSubscription(fromBinary(SubscriptionSchema, new Uint8Array(respBytes)));
}

export async function changeBillingCycleConnect(
  orgSlug: string,
  billingCycle: BillingCycle,
): Promise<{ current_cycle: string; next_cycle: string; effective_date: string }> {
  const req = create(ChangeBillingCycleRequestSchema, { orgSlug, billingCycle });
  const bytes = toBinary(ChangeBillingCycleRequestSchema, req);
  const respBytes = await getBillingService().change_cycle_connect(bytes);
  const resp = fromBinary(ChangeBillingCycleResponseSchema, new Uint8Array(respBytes));
  return {
    current_cycle: resp.currentCycle,
    next_cycle: resp.nextCycle,
    effective_date: resp.effectiveDate,
  };
}

export async function updateAutoRenewConnect(
  orgSlug: string,
  autoRenew: boolean,
): Promise<Subscription> {
  const req = create(UpdateAutoRenewRequestSchema, { orgSlug, autoRenew });
  const bytes = toBinary(UpdateAutoRenewRequestSchema, req);
  const respBytes = await getBillingService().update_auto_renew_connect(bytes);
  return fromProtoSubscription(fromBinary(SubscriptionSchema, new Uint8Array(respBytes)));
}

export async function getSeatUsageConnect(orgSlug: string): Promise<SeatUsage> {
  const req = create(GetSeatUsageRequestSchema, { orgSlug });
  const bytes = toBinary(GetSeatUsageRequestSchema, req);
  const respBytes = await getBillingService().get_seat_usage_connect(bytes);
  return fromProtoSeatUsage(fromBinary(SeatUsageSchema, new Uint8Array(respBytes)));
}

export async function purchaseSeatsConnect(
  orgSlug: string,
  seats: number,
): Promise<SeatUsage | undefined> {
  const req = create(PurchaseSeatsRequestSchema, { orgSlug, seats });
  const bytes = toBinary(PurchaseSeatsRequestSchema, req);
  const respBytes = await getBillingService().purchase_seats_connect(bytes);
  const resp = fromBinary(PurchaseSeatsResponseSchema, new Uint8Array(respBytes));
  return resp.seats ? fromProtoSeatUsage(resp.seats) : undefined;
}

export async function listInvoicesConnect(
  orgSlug: string,
  opts: { limit?: number; offset?: number } = {},
): Promise<{ items: Invoice[]; total: number; limit: number; offset: number }> {
  const req = create(ListInvoicesRequestSchema, {
    orgSlug,
    limit: opts.limit,
    offset: opts.offset,
  });
  const bytes = toBinary(ListInvoicesRequestSchema, req);
  const respBytes = await getBillingService().list_invoices_connect(bytes);
  const resp = fromBinary(ListInvoicesResponseSchema, new Uint8Array(respBytes));
  return {
    items: resp.items.map(fromProtoInvoice),
    total: Number(resp.total),
    limit: resp.limit,
    offset: resp.offset,
  };
}

export interface CreateCheckoutInput {
  order_type: OrderType;
  plan_name?: string;
  billing_cycle?: BillingCycle;
  seats?: number;
  provider?: PaymentProvider;
  success_url: string;
  cancel_url: string;
}

export async function createCheckoutConnect(
  orgSlug: string,
  input: CreateCheckoutInput,
): Promise<CheckoutResponse> {
  const req = create(CreateCheckoutRequestSchema, {
    orgSlug,
    orderType: input.order_type,
    planName: input.plan_name,
    billingCycle: input.billing_cycle,
    seats: input.seats,
    provider: input.provider,
    successUrl: input.success_url,
    cancelUrl: input.cancel_url,
  });
  const bytes = toBinary(CreateCheckoutRequestSchema, req);
  const respBytes = await getBillingService().create_checkout_connect(bytes);
  const resp = fromBinary(CreateCheckoutResponseSchema, new Uint8Array(respBytes));
  return {
    order_no: resp.orderNo,
    session_id: resp.sessionId,
    session_url: resp.sessionUrl,
    qr_code_url: resp.qrCodeUrl,
    expires_at: resp.expiresAt,
    provider: (resp.provider || undefined) as PaymentProvider | undefined,
  };
}

export async function getCheckoutStatusConnect(
  orgSlug: string,
  orderNo: string,
): Promise<CheckoutStatus> {
  const req = create(GetCheckoutStatusRequestSchema, { orgSlug, orderNo });
  const bytes = toBinary(GetCheckoutStatusRequestSchema, req);
  const respBytes = await getBillingService().get_checkout_status_connect(bytes);
  return fromProtoCheckoutStatus(fromBinary(CheckoutStatusSchema, new Uint8Array(respBytes)));
}

export async function getDeploymentInfoConnect(orgSlug: string): Promise<DeploymentInfo> {
  const req = create(GetDeploymentInfoRequestSchema, { orgSlug });
  const bytes = toBinary(GetDeploymentInfoRequestSchema, req);
  const respBytes = await getBillingService().get_deployment_info_connect(bytes);
  return fromProtoDeployment(fromBinary(DeploymentInfoSchema, new Uint8Array(respBytes)));
}

// ============== BillingPublicService — no org_slug, no auth ==============

export async function getPublicPricingConnect(currency?: string): Promise<PublicPricingResponse> {
  const req = create(GetPublicPricingRequestSchema, { currency });
  const bytes = toBinary(GetPublicPricingRequestSchema, req);
  const respBytes = await getBillingService().get_public_pricing_connect(bytes);
  return fromProtoPublicPricing(fromBinary(PublicPricingResponseSchema, new Uint8Array(respBytes)));
}

export async function getPublicDeploymentInfoConnect(): Promise<DeploymentInfo> {
  const req = create(GetPublicDeploymentInfoRequestSchema, {});
  const bytes = toBinary(GetPublicDeploymentInfoRequestSchema, req);
  const respBytes = await getBillingService().get_public_deployment_info_connect(bytes);
  return fromProtoDeployment(fromBinary(DeploymentInfoSchema, new Uint8Array(respBytes)));
}
