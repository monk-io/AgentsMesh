use crate::ApiClient;
use crate::connect_call::connect_call;
use crate::error::ApiError;
use agentsmesh_types::*;
use agentsmesh_types::proto_billing_v1 as billing_proto;

// =============================================================================
// Connect-RPC (binary wire). See proto-naming-conventions.md §2.5.
// =============================================================================
//
// These methods call the Connect handlers in backend/internal/api/connect/billing/.
// Procedure paths derive from `proto.billing.v1.BillingService.<Method>` (§12).
//
// Public service (BillingPublicService) methods live below the authenticated
// block — same procedure-path convention, but the handler bypasses
// ResolveOrgScope per conventions §3.5 (landing-page pricing card).

impl ApiClient {
    pub async fn get_billing_overview_connect(
        &self,
        req: &billing_proto::GetOverviewRequest,
    ) -> Result<billing_proto::BillingOverview, ApiError> {
        connect_call(self, "/proto.billing.v1.BillingService/GetOverview", req).await
    }

    pub async fn list_billing_plans_connect(
        &self,
        req: &billing_proto::ListPlansRequest,
    ) -> Result<billing_proto::ListPlansResponse, ApiError> {
        connect_call(self, "/proto.billing.v1.BillingService/ListPlans", req).await
    }

    pub async fn get_billing_subscription_connect(
        &self,
        req: &billing_proto::GetSubscriptionRequest,
    ) -> Result<billing_proto::Subscription, ApiError> {
        connect_call(self, "/proto.billing.v1.BillingService/GetSubscription", req).await
    }

    pub async fn create_billing_subscription_connect(
        &self,
        req: &billing_proto::CreateSubscriptionRequest,
    ) -> Result<billing_proto::Subscription, ApiError> {
        connect_call(self, "/proto.billing.v1.BillingService/CreateSubscription", req).await
    }

    pub async fn update_billing_subscription_connect(
        &self,
        req: &billing_proto::UpdateSubscriptionRequest,
    ) -> Result<billing_proto::Subscription, ApiError> {
        connect_call(self, "/proto.billing.v1.BillingService/UpdateSubscription", req).await
    }

    pub async fn cancel_billing_subscription_connect(
        &self,
        req: &billing_proto::CancelSubscriptionRequest,
    ) -> Result<billing_proto::CancelSubscriptionResponse, ApiError> {
        connect_call(self, "/proto.billing.v1.BillingService/CancelSubscription", req).await
    }

    pub async fn request_cancel_subscription_connect(
        &self,
        req: &billing_proto::RequestCancelSubscriptionRequest,
    ) -> Result<billing_proto::RequestCancelSubscriptionResponse, ApiError> {
        connect_call(
            self,
            "/proto.billing.v1.BillingService/RequestCancelSubscription",
            req,
        )
        .await
    }

    pub async fn reactivate_subscription_connect(
        &self,
        req: &billing_proto::ReactivateSubscriptionRequest,
    ) -> Result<billing_proto::Subscription, ApiError> {
        connect_call(
            self,
            "/proto.billing.v1.BillingService/ReactivateSubscription",
            req,
        )
        .await
    }

    pub async fn upgrade_subscription_connect(
        &self,
        req: &billing_proto::UpgradeSubscriptionRequest,
    ) -> Result<billing_proto::Subscription, ApiError> {
        connect_call(
            self,
            "/proto.billing.v1.BillingService/UpgradeSubscription",
            req,
        )
        .await
    }

    pub async fn change_billing_cycle_connect(
        &self,
        req: &billing_proto::ChangeBillingCycleRequest,
    ) -> Result<billing_proto::ChangeBillingCycleResponse, ApiError> {
        connect_call(
            self,
            "/proto.billing.v1.BillingService/ChangeBillingCycle",
            req,
        )
        .await
    }

    pub async fn update_auto_renew_connect(
        &self,
        req: &billing_proto::UpdateAutoRenewRequest,
    ) -> Result<billing_proto::Subscription, ApiError> {
        connect_call(self, "/proto.billing.v1.BillingService/UpdateAutoRenew", req).await
    }

    pub async fn get_seat_usage_connect(
        &self,
        req: &billing_proto::GetSeatUsageRequest,
    ) -> Result<billing_proto::SeatUsage, ApiError> {
        connect_call(self, "/proto.billing.v1.BillingService/GetSeatUsage", req).await
    }

    pub async fn purchase_seats_connect(
        &self,
        req: &billing_proto::PurchaseSeatsRequest,
    ) -> Result<billing_proto::PurchaseSeatsResponse, ApiError> {
        connect_call(self, "/proto.billing.v1.BillingService/PurchaseSeats", req).await
    }

    pub async fn list_billing_invoices_connect(
        &self,
        req: &billing_proto::ListInvoicesRequest,
    ) -> Result<billing_proto::ListInvoicesResponse, ApiError> {
        connect_call(self, "/proto.billing.v1.BillingService/ListInvoices", req).await
    }

    pub async fn create_billing_checkout_connect(
        &self,
        req: &billing_proto::CreateCheckoutRequest,
    ) -> Result<billing_proto::CreateCheckoutResponse, ApiError> {
        connect_call(self, "/proto.billing.v1.BillingService/CreateCheckout", req).await
    }

    pub async fn get_billing_checkout_status_connect(
        &self,
        req: &billing_proto::GetCheckoutStatusRequest,
    ) -> Result<billing_proto::CheckoutStatus, ApiError> {
        connect_call(
            self,
            "/proto.billing.v1.BillingService/GetCheckoutStatus",
            req,
        )
        .await
    }

    pub async fn get_billing_deployment_info_connect(
        &self,
        req: &billing_proto::GetDeploymentInfoRequest,
    ) -> Result<billing_proto::DeploymentInfo, ApiError> {
        connect_call(
            self,
            "/proto.billing.v1.BillingService/GetDeploymentInfo",
            req,
        )
        .await
    }

    // BillingPublicService — no org_slug, no auth interceptor on the backend
    // (conventions §3.5 exception). Same wire format; just a different
    // service stem.

    pub async fn get_public_pricing_connect(
        &self,
        req: &billing_proto::GetPublicPricingRequest,
    ) -> Result<billing_proto::PublicPricingResponse, ApiError> {
        connect_call(
            self,
            "/proto.billing.v1.BillingPublicService/GetPublicPricing",
            req,
        )
        .await
    }

    pub async fn get_public_deployment_info_connect(
        &self,
        req: &billing_proto::GetPublicDeploymentInfoRequest,
    ) -> Result<billing_proto::DeploymentInfo, ApiError> {
        connect_call(
            self,
            "/proto.billing.v1.BillingPublicService/GetPublicDeploymentInfo",
            req,
        )
        .await
    }
}

// =============================================================================
// Legacy REST methods — preserved for dual-track migration.
// =============================================================================

impl ApiClient {
    pub async fn get_billing_overview(&self) -> Result<BillingOverview, ApiError> {
        self.get_resource(&self.org_path("/billing/overview"), "overview").await
    }

    pub async fn get_billing_subscription(&self) -> Result<Subscription, ApiError> {
        self.get_resource(&self.org_path("/billing/subscription"), "subscription").await
    }

    pub async fn create_billing_subscription(
        &self,
        data: &CreateSubscriptionRequest,
    ) -> Result<Subscription, ApiError> {
        self.post_resource(&self.org_path("/billing/subscription"), data, "subscription").await
    }

    pub async fn update_billing_subscription(
        &self,
        data: &UpdateSubscriptionRequest,
    ) -> Result<Subscription, ApiError> {
        self.put_resource(&self.org_path("/billing/subscription"), data, "subscription").await
    }

    pub async fn cancel_billing_subscription(&self) -> Result<EmptyResponse, ApiError> {
        self.delete(&self.org_path("/billing/subscription")).await
    }

    pub async fn list_billing_plans(&self) -> Result<PlanListResponse, ApiError> {
        self.get(&self.org_path("/billing/plans")).await
    }

    pub async fn get_billing_usage(
        &self,
        usage_type: Option<&str>,
    ) -> Result<BillingUsageResponse, ApiError> {
        let mut path = self.org_path("/billing/usage");
        if let Some(t) = usage_type {
            path = format!("{path}?type={t}");
        }
        self.get(&path).await
    }

    pub async fn check_billing_quota(
        &self,
        resource: &str,
        amount: Option<u32>,
    ) -> Result<QuotaCheckResponse, ApiError> {
        let mut path = self.org_path(&format!("/billing/quota/check?resource={resource}"));
        if let Some(a) = amount {
            path = format!("{path}&amount={a}");
        }
        self.get(&path).await
    }

    pub async fn create_billing_checkout(
        &self,
        data: &CheckoutRequest,
    ) -> Result<CheckoutStatus, ApiError> {
        self.post(&self.org_path("/billing/checkout"), data).await
    }

    pub async fn get_billing_checkout_status(
        &self,
        order_no: &str,
    ) -> Result<CheckoutStatus, ApiError> {
        self.get(&self.org_path(&format!("/billing/checkout/{order_no}")))
            .await
    }

    pub async fn request_cancel_subscription(
        &self,
        data: &CancelSubscriptionRequest,
    ) -> Result<EmptyResponse, ApiError> {
        self.post(
            &self.org_path("/billing/subscription/cancel"),
            data,
        )
        .await
    }

    pub async fn reactivate_subscription(&self) -> Result<Subscription, ApiError> {
        self.post(
            &self.org_path("/billing/subscription/reactivate"),
            &serde_json::json!({}),
        )
        .await
    }

    pub async fn upgrade_subscription(
        &self,
        data: &UpgradeSubscriptionRequest,
    ) -> Result<Subscription, ApiError> {
        self.post(
            &self.org_path("/billing/subscription/upgrade"),
            data,
        )
        .await
    }

    pub async fn change_billing_cycle(
        &self,
        data: &ChangeBillingCycleRequest,
    ) -> Result<Subscription, ApiError> {
        self.post(
            &self.org_path("/billing/subscription/change-cycle"),
            data,
        )
        .await
    }

    pub async fn update_auto_renew(
        &self,
        data: &UpdateAutoRenewRequest,
    ) -> Result<Subscription, ApiError> {
        self.put(
            &self.org_path("/billing/subscription/auto-renew"),
            data,
        )
        .await
    }

    pub async fn get_seat_usage(&self) -> Result<SeatUsage, ApiError> {
        self.get(&self.org_path("/billing/seats")).await
    }

    pub async fn purchase_seats(
        &self,
        data: &PurchaseSeatsRequest,
    ) -> Result<EmptyResponse, ApiError> {
        self.post(&self.org_path("/billing/seats/purchase"), data)
            .await
    }

    pub async fn list_billing_invoices(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<InvoiceListResponse, ApiError> {
        let mut path = self.org_path("/billing/invoices");
        let mut params = Vec::new();
        if let Some(l) = limit {
            params.push(format!("limit={l}"));
        }
        if let Some(o) = offset {
            params.push(format!("offset={o}"));
        }
        if !params.is_empty() {
            path = format!("{path}?{}", params.join("&"));
        }
        self.get(&path).await
    }

    pub async fn get_customer_portal(
        &self,
        data: &CustomerPortalRequest,
    ) -> Result<CustomerPortalResponse, ApiError> {
        self.post(&self.org_path("/billing/customer-portal"), data)
            .await
    }

    pub async fn get_billing_deployment_info(&self) -> Result<DeploymentInfo, ApiError> {
        self.get(&self.org_path("/billing/deployment")).await
    }
}
