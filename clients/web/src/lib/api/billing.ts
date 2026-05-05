import { getBillingService } from "@/lib/wasm-core";
export type {
  SubscriptionPlan, PlanPrice, PlanWithPrice, UsageOverview, BillingOverview,
  Subscription, CheckoutRequest, CheckoutResponse, CheckoutStatus,
  SeatUsage, Invoice, DeploymentInfo, PublicPlanPricing, PublicPricingResponse,
  Currency, BillingCycle, OrderType, PaymentProvider,
} from "./billing-types";

export const billingApi = {
  getOverview: async () => {
    const json = await getBillingService().get_overview();
    return JSON.parse(json);
  },
  listPlans: async () => {
    const json = await getBillingService().list_plans();
    return JSON.parse(json);
  },
  getDeploymentInfo: async () => {
    const json = await getBillingService().get_deployment_info();
    return JSON.parse(json);
  },
  createSubscription: async (planName: string) => {
    const json = await getBillingService().create_subscription(JSON.stringify({ plan_name: planName }));
    return JSON.parse(json);
  },
  updateSubscription: async (planName: string) => {
    const json = await getBillingService().update_subscription(JSON.stringify({ plan_name: planName }));
    return JSON.parse(json);
  },
  upgradeSubscription: async (planName: string) => {
    const json = await getBillingService().upgrade(JSON.stringify({ plan_name: planName }));
    return JSON.parse(json);
  },
  reactivateSubscription: async () => {
    const json = await getBillingService().reactivate();
    return JSON.parse(json);
  },
  requestCancelSubscription: async (immediate: boolean) => {
    const json = await getBillingService().request_cancel(JSON.stringify({ immediate }));
    return JSON.parse(json);
  },
  listInvoices: async (limit?: number, offset?: number) => {
    const json = await getBillingService().list_invoices(limit ?? null, offset ?? null);
    return JSON.parse(json);
  },
  createCheckout: async (data: Record<string, unknown>) => {
    const json = await getBillingService().create_checkout(JSON.stringify(data));
    return JSON.parse(json);
  },
  getCheckoutStatus: async (orderNo: string) => {
    const json = await getBillingService().get_checkout_status(orderNo);
    return JSON.parse(json);
  },
  changeBillingCycle: async (cycle: string) => {
    const json = await getBillingService().change_cycle(JSON.stringify({ cycle }));
    return JSON.parse(json);
  },
  getSeatUsage: async () => {
    const json = await getBillingService().get_seat_usage();
    return JSON.parse(json);
  },
  purchaseSeats: async (count: number) => {
    const json = await getBillingService().purchase_seats(JSON.stringify({ count }));
    return JSON.parse(json);
  },
};
