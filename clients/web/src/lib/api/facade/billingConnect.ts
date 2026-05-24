// Facade re-export of the billing Connect-RPC adapter. Business code imports
// from here (or from the `@/lib/api` barrel) so the wire-shape layer stays
// internal to the facade boundary. Tests mock this path.

export {
  getOverviewConnect,
  listPlansConnect,
  getSubscriptionConnect,
  createSubscriptionConnect,
  updateSubscriptionConnect,
  cancelSubscriptionConnect,
  requestCancelSubscriptionConnect,
  reactivateSubscriptionConnect,
  upgradeSubscriptionConnect,
  changeBillingCycleConnect,
  updateAutoRenewConnect,
  getSeatUsageConnect,
  purchaseSeatsConnect,
  listInvoicesConnect,
  createCheckoutConnect,
  getCheckoutStatusConnect,
  getDeploymentInfoConnect,
  getPublicPricingConnect,
  getPublicDeploymentInfoConnect,
  getUsageConnect,
  getUsageHistoryConnect,
  checkQuotaConnect,
  setCustomQuotaConnect,
  createCustomerPortalConnect,
  type CreateCheckoutInput,
} from "../connect/billingConnect";
