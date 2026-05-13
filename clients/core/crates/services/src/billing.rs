use std::sync::Arc;

use agentsmesh_api_client::ApiClient;
use agentsmesh_types::*;
use agentsmesh_types::proto_billing_v1 as billing_proto;
use prost::Message;

// Project proto.billing.v1.Subscription onto the legacy serde Subscription
// shape (`{plan_name, status, billing_cycle, period boundaries, auto_renew,
// seats, cancel_at_period_end}`). Provider IDs (Stripe / LemonSqueezy) and
// the nested `plan` SubscriptionPlan are intentionally dropped — the legacy
// DTO never carried them and existing callers don't read them.
fn proto_subscription_to_legacy(s: &billing_proto::Subscription) -> serde_json::Value {
    serde_json::json!({
        "plan_name": s.plan.as_ref().map(|p| p.name.clone()),
        "status": option_str_to_value(&s.status),
        "billing_cycle": option_str_to_value(&s.billing_cycle),
        "current_period_start": option_str_to_value(&s.current_period_start),
        "current_period_end": option_str_to_value(&s.current_period_end),
        "auto_renew": s.auto_renew,
        "seats": s.seat_count,
        "cancel_at_period_end": s.cancel_at_period_end,
    })
}

fn option_str_to_value(s: &str) -> serde_json::Value {
    if s.is_empty() {
        serde_json::Value::Null
    } else {
        serde_json::Value::String(s.to_string())
    }
}

pub struct BillingService {
    client: Arc<ApiClient>,
}

impl BillingService {
    pub fn new(client: Arc<ApiClient>) -> Self {
        Self { client }
    }

    pub async fn get_overview(&self) -> Result<String, String> {
        let req = billing_proto::GetOverviewRequest {
            org_slug: self.client.current_org_slug(),
        };
        let resp = self.client.get_billing_overview_connect(&req).await.map_err(crate::wire)?;
        // Legacy BillingOverview shape: { subscription, usage_summary, seats }.
        // Proto BillingOverview has plan + status/cycle/period + usage. Reconstruct
        // a subscription-shaped object out of the proto fields so existing call
        // sites that read `.subscription.plan_name` keep working.
        let subscription = serde_json::json!({
            "plan_name": resp.plan.as_ref().map(|p| p.name.clone()),
            "status": option_str_to_value(&resp.status),
            "billing_cycle": option_str_to_value(&resp.billing_cycle),
            "current_period_start": option_str_to_value(&resp.current_period_start),
            "current_period_end": option_str_to_value(&resp.current_period_end),
            "auto_renew": serde_json::Value::Null,
            "seats": serde_json::Value::Null,
            "cancel_at_period_end": resp.cancel_at_period_end,
        });
        let usage_summary = resp.usage.as_ref().map(|u| serde_json::json!({
            "pod_minutes": u.pod_minutes,
            "included_pod_minutes": u.included_pod_minutes,
            "users": u.users,
            "max_users": u.max_users,
            "runners": u.runners,
            "max_runners": u.max_runners,
            "concurrent_pods": u.concurrent_pods,
            "max_concurrent_pods": u.max_concurrent_pods,
            "repositories": u.repositories,
            "max_repositories": u.max_repositories,
        }));
        let envelope = serde_json::json!({
            "subscription": subscription,
            "usage_summary": usage_summary,
            "seats": serde_json::Value::Null,
        });
        serde_json::to_string(&envelope).map_err(crate::wire)
    }

    pub async fn get_subscription(&self) -> Result<String, String> {
        let req = billing_proto::GetSubscriptionRequest {
            org_slug: self.client.current_org_slug(),
        };
        let resp = self.client.get_billing_subscription_connect(&req).await.map_err(crate::wire)?;
        let sub = proto_subscription_to_legacy(&resp);
        serde_json::to_string(&sub).map_err(crate::wire)
    }

    pub async fn create_subscription(&self, json: &str) -> Result<String, String> {
        let req_legacy: CreateSubscriptionRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let req = billing_proto::CreateSubscriptionRequest {
            org_slug: self.client.current_org_slug(),
            plan_name: req_legacy.plan_name,
            billing_cycle: req_legacy.billing_cycle,
        };
        let resp = self.client.create_billing_subscription_connect(&req).await.map_err(crate::wire)?;
        let sub = proto_subscription_to_legacy(&resp);
        serde_json::to_string(&sub).map_err(crate::wire)
    }

    pub async fn cancel_subscription(&self) -> Result<String, String> {
        let req = billing_proto::CancelSubscriptionRequest {
            org_slug: self.client.current_org_slug(),
        };
        self.client.cancel_billing_subscription_connect(&req).await.map_err(crate::wire)?;
        // Legacy returned EmptyResponse → serialized as `null`.
        Ok("null".to_string())
    }

    pub async fn update_subscription(&self, json: &str) -> Result<String, String> {
        let req_legacy: UpdateSubscriptionRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let req = billing_proto::UpdateSubscriptionRequest {
            org_slug: self.client.current_org_slug(),
            plan_name: req_legacy.plan_name,
        };
        let resp = self.client.update_billing_subscription_connect(&req).await.map_err(crate::wire)?;
        let sub = proto_subscription_to_legacy(&resp);
        serde_json::to_string(&sub).map_err(crate::wire)
    }

    pub async fn list_plans(&self) -> Result<String, String> {
        let req = billing_proto::ListPlansRequest {
            org_slug: self.client.current_org_slug(),
        };
        let resp = self.client.list_billing_plans_connect(&req).await.map_err(crate::wire)?;
        let plans: Vec<serde_json::Value> = resp.items.into_iter().map(|p| serde_json::json!({
            "name": p.name,
            "display_name": p.display_name,
            // Proto SubscriptionPlan doesn't carry `description`; legacy nullable.
            "description": serde_json::Value::Null,
            // Legacy `features` is a JSON blob — surface key plan fields under it
            // so the UI can read them via the same shape.
            "features": serde_json::json!({
                "price_per_seat_monthly": p.price_per_seat_monthly,
                "price_per_seat_yearly": p.price_per_seat_yearly,
                "included_pod_minutes": p.included_pod_minutes,
                "price_per_extra_minute": p.price_per_extra_minute,
                "max_users": p.max_users,
                "max_runners": p.max_runners,
                "max_concurrent_pods": p.max_concurrent_pods,
                "max_repositories": p.max_repositories,
            }),
            "is_active": p.is_active,
        })).collect();
        let envelope = serde_json::json!({
            "plans": plans,
            "currency": serde_json::Value::Null,
        });
        serde_json::to_string(&envelope).map_err(crate::wire)
    }

    pub async fn get_usage(&self, usage_type: Option<String>) -> Result<String, String> {
        // proto.billing.v1.BillingService doesn't carry a GetUsage RPC —
        // usage rolls up into the overview. Keep legacy REST until proto
        // adds a dedicated usage endpoint.
        let resp = self.client
            .get_billing_usage(usage_type.as_deref())
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn check_quota(&self, resource: &str, amount: Option<u32>) -> Result<String, String> {
        // Same as get_usage — no proto coverage yet, stay on REST.
        let resp = self.client
            .check_billing_quota(resource, amount)
            .await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn create_checkout(&self, json: &str) -> Result<String, String> {
        let req_legacy: CheckoutRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let req = billing_proto::CreateCheckoutRequest {
            org_slug: self.client.current_org_slug(),
            order_type: req_legacy.order_type.unwrap_or_default(),
            plan_name: Some(req_legacy.plan_name),
            billing_cycle: req_legacy.billing_cycle,
            seats: req_legacy.seats.map(|v| v as i32),
            provider: req_legacy.provider,
            success_url: req_legacy.success_url.unwrap_or_default(),
            cancel_url: req_legacy.cancel_url.unwrap_or_default(),
        };
        let resp = self.client.create_billing_checkout_connect(&req).await.map_err(crate::wire)?;
        // Legacy CheckoutStatus shape: {order_no, status, payment_url}. The
        // proto CreateCheckoutResponse returns session_url (not payment_url)
        // and exposes session_id / qr_code_url / expires_at — surface all of
        // them so the web checkout flow can pick the right URL.
        let envelope = serde_json::json!({
            "order_no": resp.order_no,
            "status": "pending",
            "payment_url": resp.session_url,
            "session_id": resp.session_id,
            "qr_code_url": resp.qr_code_url,
            "expires_at": resp.expires_at,
            "provider": resp.provider,
        });
        serde_json::to_string(&envelope).map_err(crate::wire)
    }

    pub async fn get_checkout_status(&self, order_no: &str) -> Result<String, String> {
        let req = billing_proto::GetCheckoutStatusRequest {
            org_slug: self.client.current_org_slug(),
            order_no: order_no.to_string(),
        };
        let resp = self.client.get_billing_checkout_status_connect(&req).await.map_err(crate::wire)?;
        let envelope = serde_json::json!({
            "order_no": resp.order_no,
            "status": resp.status,
            // Legacy CheckoutStatus has `payment_url`; proto carries it on
            // CreateCheckoutResponse, not here. Surface null so deserialize works.
            "payment_url": serde_json::Value::Null,
            "order_type": resp.order_type,
            "amount": resp.amount,
            "currency": resp.currency,
            "created_at": resp.created_at,
            "paid_at": resp.paid_at,
        });
        serde_json::to_string(&envelope).map_err(crate::wire)
    }

    pub async fn request_cancel(&self, json: &str) -> Result<String, String> {
        let req_legacy: CancelSubscriptionRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let req = billing_proto::RequestCancelSubscriptionRequest {
            org_slug: self.client.current_org_slug(),
            immediate: req_legacy.immediate.unwrap_or(false),
        };
        let _resp = self.client.request_cancel_subscription_connect(&req).await.map_err(crate::wire)?;
        Ok("null".to_string())
    }

    pub async fn reactivate(&self) -> Result<String, String> {
        let req = billing_proto::ReactivateSubscriptionRequest {
            org_slug: self.client.current_org_slug(),
        };
        let resp = self.client.reactivate_subscription_connect(&req).await.map_err(crate::wire)?;
        let sub = proto_subscription_to_legacy(&resp);
        serde_json::to_string(&sub).map_err(crate::wire)
    }

    pub async fn upgrade(&self, json: &str) -> Result<String, String> {
        let req_legacy: UpgradeSubscriptionRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let req = billing_proto::UpgradeSubscriptionRequest {
            org_slug: self.client.current_org_slug(),
            plan_name: req_legacy.plan_name,
        };
        let resp = self.client.upgrade_subscription_connect(&req).await.map_err(crate::wire)?;
        let sub = proto_subscription_to_legacy(&resp);
        serde_json::to_string(&sub).map_err(crate::wire)
    }

    pub async fn change_cycle(&self, json: &str) -> Result<String, String> {
        let req_legacy: ChangeBillingCycleRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let req = billing_proto::ChangeBillingCycleRequest {
            org_slug: self.client.current_org_slug(),
            billing_cycle: req_legacy.billing_cycle,
        };
        let _resp = self.client.change_billing_cycle_connect(&req).await.map_err(crate::wire)?;
        // Legacy returned the updated Subscription; the proto response carries
        // {current_cycle, next_cycle, effective_date}. Refetch to keep contract.
        self.get_subscription().await
    }

    pub async fn update_auto_renew(&self, json: &str) -> Result<String, String> {
        let req_legacy: UpdateAutoRenewRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let req = billing_proto::UpdateAutoRenewRequest {
            org_slug: self.client.current_org_slug(),
            auto_renew: req_legacy.auto_renew,
        };
        let resp = self.client.update_auto_renew_connect(&req).await.map_err(crate::wire)?;
        let sub = proto_subscription_to_legacy(&resp);
        serde_json::to_string(&sub).map_err(crate::wire)
    }

    pub async fn get_seat_usage(&self) -> Result<String, String> {
        let req = billing_proto::GetSeatUsageRequest {
            org_slug: self.client.current_org_slug(),
        };
        let resp = self.client.get_seat_usage_connect(&req).await.map_err(crate::wire)?;
        // Legacy SeatUsage shape: {total, used, available}. Proto carries
        // {total_seats, used_seats, available_seats, max_seats, can_add_seats}.
        let envelope = serde_json::json!({
            "total": resp.total_seats,
            "used": resp.used_seats,
            "available": resp.available_seats,
            "max_seats": resp.max_seats,
            "can_add_seats": resp.can_add_seats,
        });
        serde_json::to_string(&envelope).map_err(crate::wire)
    }

    pub async fn purchase_seats(&self, json: &str) -> Result<String, String> {
        let req_legacy: PurchaseSeatsRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let req = billing_proto::PurchaseSeatsRequest {
            org_slug: self.client.current_org_slug(),
            seats: req_legacy.seats as i32,
        };
        let _resp = self.client.purchase_seats_connect(&req).await.map_err(crate::wire)?;
        // Legacy returned EmptyResponse.
        Ok("null".to_string())
    }

    pub async fn list_invoices(
        &self, limit: Option<u32>, offset: Option<u32>,
    ) -> Result<String, String> {
        let req = billing_proto::ListInvoicesRequest {
            org_slug: self.client.current_org_slug(),
            offset: offset.map(|v| v as i32),
            limit: limit.map(|v| v as i32),
        };
        let resp = self.client.list_billing_invoices_connect(&req).await.map_err(crate::wire)?;
        // Legacy Invoice shape: {id (String!), amount, currency, status,
        // invoice_url, created_at}. Proto has int64 id + invoice_no (string)
        // + status/currency/total + pdf_url + issued_at — map onto the legacy
        // shape so the existing UI table doesn't have to change.
        let invoices: Vec<serde_json::Value> = resp.items.into_iter().map(|inv| serde_json::json!({
            "id": inv.invoice_no,
            "amount": inv.total,
            "currency": inv.currency,
            "status": inv.status,
            "invoice_url": inv.pdf_url,
            "created_at": inv.issued_at,
        })).collect();
        let envelope = serde_json::json!({
            "invoices": invoices,
            "total": resp.total,
        });
        serde_json::to_string(&envelope).map_err(crate::wire)
    }

    pub async fn get_customer_portal(&self, json: &str) -> Result<String, String> {
        // No proto coverage — Stripe/LemonSqueezy customer portal is a
        // provider-side redirect that the backend mints, not a domain RPC.
        let req: CustomerPortalRequest = serde_json::from_str(json).map_err(crate::wire)?;
        let resp = self.client.get_customer_portal(&req).await.map_err(crate::wire)?;
        serde_json::to_string(&resp).map_err(crate::wire)
    }

    pub async fn get_deployment_info(&self) -> Result<String, String> {
        let req = billing_proto::GetDeploymentInfoRequest {
            org_slug: self.client.current_org_slug(),
        };
        let resp = self.client.get_billing_deployment_info_connect(&req).await.map_err(crate::wire)?;
        // Legacy DeploymentInfo shape: {billing_enabled, payment_providers}.
        // Proto DeploymentInfo carries {deployment_type, available_providers}.
        // billing_enabled mirrors !providers.is_empty() — same business rule
        // the REST handler enforced.
        let envelope = serde_json::json!({
            "billing_enabled": !resp.available_providers.is_empty(),
            "payment_providers": resp.available_providers,
            "deployment_type": resp.deployment_type,
        });
        serde_json::to_string(&envelope).map_err(crate::wire)
    }

    pub async fn get_public_pricing(&self) -> Result<String, String> {
        let req = billing_proto::GetPublicPricingRequest { currency: None };
        let resp = self.client.get_public_pricing_connect(&req).await.map_err(crate::wire)?;
        // Match legacy PublicPricingResponse (deployment_type/currency/plans).
        let envelope = serde_json::json!({
            "deployment_type": resp.deployment_type,
            "currency": resp.currency,
            "plans": resp.plans.into_iter().map(|p| serde_json::json!({
                "name": p.name,
                "display_name": p.display_name,
                "price_monthly": p.price_monthly,
                "price_yearly": p.price_yearly,
                "max_users": p.max_users,
                "max_runners": p.max_runners,
                "max_repositories": p.max_repositories,
                "max_concurrent_pods": p.max_concurrent_pods,
            })).collect::<Vec<_>>(),
        });
        serde_json::to_string(&envelope).map_err(crate::wire)
    }

    pub async fn get_public_deployment_info(&self) -> Result<String, String> {
        let req = billing_proto::GetPublicDeploymentInfoRequest {};
        let resp = self.client.get_public_deployment_info_connect(&req).await.map_err(crate::wire)?;
        let envelope = serde_json::json!({
            "billing_enabled": !resp.available_providers.is_empty(),
            "payment_providers": resp.available_providers,
            "deployment_type": resp.deployment_type,
        });
        serde_json::to_string(&envelope).map_err(crate::wire)
    }

    // -------- Connect-RPC (binary wire) --------
    //
    // Each `*_connect` method takes prost-encoded bytes and returns
    // prost-encoded bytes — matching the wasm bridge's `Result<Vec<u8>, String>`
    // surface (conventions §2.5). Caller (TS) encodes via
    // @bufbuild/protobuf .toBinary() and decodes via .fromBinary().

    pub async fn get_overview_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::GetOverviewRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_overview request: {e}"))?;
        let resp = self.client.get_billing_overview_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn list_plans_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::ListPlansRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_plans request: {e}"))?;
        let resp = self.client.list_billing_plans_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_subscription_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::GetSubscriptionRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_subscription request: {e}"))?;
        let resp = self.client.get_billing_subscription_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn create_subscription_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::CreateSubscriptionRequest::decode(request_bytes)
            .map_err(|e| format!("decode create_subscription request: {e}"))?;
        let resp = self.client.create_billing_subscription_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn update_subscription_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::UpdateSubscriptionRequest::decode(request_bytes)
            .map_err(|e| format!("decode update_subscription request: {e}"))?;
        let resp = self.client.update_billing_subscription_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn cancel_subscription_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::CancelSubscriptionRequest::decode(request_bytes)
            .map_err(|e| format!("decode cancel_subscription request: {e}"))?;
        let resp = self.client.cancel_billing_subscription_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn request_cancel_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::RequestCancelSubscriptionRequest::decode(request_bytes)
            .map_err(|e| format!("decode request_cancel request: {e}"))?;
        let resp = self.client.request_cancel_subscription_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn reactivate_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::ReactivateSubscriptionRequest::decode(request_bytes)
            .map_err(|e| format!("decode reactivate request: {e}"))?;
        let resp = self.client.reactivate_subscription_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn upgrade_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::UpgradeSubscriptionRequest::decode(request_bytes)
            .map_err(|e| format!("decode upgrade request: {e}"))?;
        let resp = self.client.upgrade_subscription_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn change_cycle_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::ChangeBillingCycleRequest::decode(request_bytes)
            .map_err(|e| format!("decode change_cycle request: {e}"))?;
        let resp = self.client.change_billing_cycle_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn update_auto_renew_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::UpdateAutoRenewRequest::decode(request_bytes)
            .map_err(|e| format!("decode update_auto_renew request: {e}"))?;
        let resp = self.client.update_auto_renew_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_seat_usage_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::GetSeatUsageRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_seat_usage request: {e}"))?;
        let resp = self.client.get_seat_usage_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn purchase_seats_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::PurchaseSeatsRequest::decode(request_bytes)
            .map_err(|e| format!("decode purchase_seats request: {e}"))?;
        let resp = self.client.purchase_seats_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn list_invoices_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::ListInvoicesRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_invoices request: {e}"))?;
        let resp = self.client.list_billing_invoices_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn create_checkout_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::CreateCheckoutRequest::decode(request_bytes)
            .map_err(|e| format!("decode create_checkout request: {e}"))?;
        let resp = self.client.create_billing_checkout_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_checkout_status_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::GetCheckoutStatusRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_checkout_status request: {e}"))?;
        let resp = self.client.get_billing_checkout_status_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_deployment_info_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::GetDeploymentInfoRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_deployment_info request: {e}"))?;
        let resp = self.client.get_billing_deployment_info_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_public_pricing_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::GetPublicPricingRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_public_pricing request: {e}"))?;
        let resp = self.client.get_public_pricing_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_public_deployment_info_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = billing_proto::GetPublicDeploymentInfoRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_public_deployment_info request: {e}"))?;
        let resp = self.client.get_public_deployment_info_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }
}
