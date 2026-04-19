// Auto-generated from tauri-bridge — DO NOT edit manually
// Regenerate with: python3 scripts/gen-node-bridge.py

use napi_derive::napi;
use crate::{AppState, err};

#[napi]
impl AppState {
    // ===== agent.rs =====
    #[napi]
    pub async fn agent_list_agents(&self) -> napi::Result<String> {
        let svc = self.agent.lock().await;
            svc.list_agents().await.map_err(err)
    }

    #[napi]
    pub async fn agent_get_config_schema(&self, agent_slug: String) -> napi::Result<String> {
        let svc = self.agent.lock().await;
            svc.get_config_schema(&agent_slug).await.map_err(err)
    }

    #[napi]
    pub async fn agent_list_user_configs(&self) -> napi::Result<String> {
        let svc = self.agent.lock().await;
            svc.list_user_configs().await.map_err(err)
    }

    #[napi]
    pub async fn agent_get_user_config(&self, agent_slug: String) -> napi::Result<String> {
        let svc = self.agent.lock().await;
            svc.get_user_config(&agent_slug).await.map_err(err)
    }

    #[napi]
    pub async fn agent_set_user_config(&self, agent_slug: String, json: String) -> napi::Result<String> {
        let svc = self.agent.lock().await;
            svc.set_user_config(&agent_slug, &json).await.map_err(err)
    }

    #[napi]
    pub async fn agent_delete_user_config(&self, agent_slug: String) -> napi::Result<()> {
        let svc = self.agent.lock().await;
            svc.delete_user_config(&agent_slug).await.map_err(err)
    }

    #[napi]
    pub async fn agent_get_agentpod_settings(&self) -> napi::Result<String> {
        let svc = self.agent.lock().await;
            svc.get_agentpod_settings().await.map_err(err)
    }

    #[napi]
    pub async fn agent_update_agentpod_settings(&self, json: String) -> napi::Result<String> {
        let svc = self.agent.lock().await;
            svc.update_agentpod_settings(&json).await.map_err(err)
    }

    #[napi]
    pub async fn agent_list_providers(&self) -> napi::Result<String> {
        let svc = self.agent.lock().await;
            svc.list_providers().await.map_err(err)
    }

    #[napi]
    pub async fn agent_create_provider(&self, json: String) -> napi::Result<String> {
        let svc = self.agent.lock().await;
            svc.create_provider(&json).await.map_err(err)
    }

    #[napi]
    pub async fn agent_update_provider(&self, id: i64, json: String) -> napi::Result<String> {
        let svc = self.agent.lock().await;
            svc.update_provider(id, &json).await.map_err(err)
    }

    #[napi]
    pub async fn agent_delete_provider(&self, id: i64) -> napi::Result<()> {
        let svc = self.agent.lock().await;
            svc.delete_provider(id).await.map_err(err)
    }

    #[napi]
    pub async fn agent_set_default_provider(&self, id: i64) -> napi::Result<()> {
        let svc = self.agent.lock().await;
            svc.set_default_provider(id).await.map_err(err)
    }

    // ===== apikey.rs =====
    #[napi]
    pub async fn apikey_list(&self) -> napi::Result<String> {
        let svc = self.apikey.lock().await;
            svc.list().await.map_err(err)
    }

    #[napi]
    pub async fn apikey_get(&self, id: i64) -> napi::Result<String> {
        let svc = self.apikey.lock().await;
            svc.get(id).await.map_err(err)
    }

    #[napi]
    pub async fn apikey_create(&self, json: String) -> napi::Result<String> {
        let svc = self.apikey.lock().await;
            svc.create(&json).await.map_err(err)
    }

    #[napi]
    pub async fn apikey_update(&self, id: i64, json: String) -> napi::Result<String> {
        let svc = self.apikey.lock().await;
            svc.update(id, &json).await.map_err(err)
    }

    #[napi]
    pub async fn apikey_delete(&self, id: i64) -> napi::Result<()> {
        let svc = self.apikey.lock().await;
            svc.delete(id).await.map_err(err)
    }

    #[napi]
    pub async fn apikey_revoke(&self, id: i64) -> napi::Result<()> {
        let svc = self.apikey.lock().await;
            svc.revoke(id).await.map_err(err)
    }

    // ===== auth_api.rs =====
    #[napi]
    pub async fn auth_api_register(&self, json: String) -> napi::Result<String> {
        let svc = self.auth_api.lock().await;
            svc.register(&json).await.map_err(err)
    }

    #[napi]
    pub async fn auth_api_verify_email(&self, token: String) -> napi::Result<String> {
        let svc = self.auth_api.lock().await;
            svc.verify_email(&token).await.map_err(err)
    }

    #[napi]
    pub async fn auth_api_resend_verification(&self, email: String) -> napi::Result<String> {
        let svc = self.auth_api.lock().await;
            svc.resend_verification(&email).await.map_err(err)
    }

    #[napi]
    pub async fn auth_api_forgot_password(&self, email: String) -> napi::Result<String> {
        let svc = self.auth_api.lock().await;
            svc.forgot_password(&email).await.map_err(err)
    }

    #[napi]
    pub async fn auth_api_reset_password(&self, json: String) -> napi::Result<String> {
        let svc = self.auth_api.lock().await;
            svc.reset_password(&json).await.map_err(err)
    }

    // ===== autopilot.rs =====
    #[napi]
    pub async fn autopilot_controllers_json(&self) -> napi::Result<String> {
        let svc = self.autopilot.lock().await;
            Ok(svc.controllers_json())
    }

    #[napi]
    pub async fn autopilot_current_controller_json(&self) -> napi::Result<String> {
        let svc = self.autopilot.lock().await;
            Ok(svc.current_controller_json().unwrap_or_default())
    }

    #[napi]
    pub async fn autopilot_get_controller_by_pod_key_json(&self, pod_key: String) -> napi::Result<String> {
        let svc = self.autopilot.lock().await;
            Ok(svc.get_controller_by_pod_key_json(&pod_key).unwrap_or_default())
    }

    #[napi]
    pub async fn autopilot_get_iterations_json(&self, key: String) -> napi::Result<String> {
        let svc = self.autopilot.lock().await;
            Ok(svc.get_iterations_json(&key).unwrap_or_default())
    }

    #[napi]
    pub async fn autopilot_get_thinking_json(&self, key: String) -> napi::Result<String> {
        let svc = self.autopilot.lock().await;
            Ok(svc.get_thinking_json(&key).unwrap_or_default())
    }

    #[napi]
    pub async fn autopilot_get_thinking_history_json(&self, key: String) -> napi::Result<String> {
        let svc = self.autopilot.lock().await;
            Ok(svc.get_thinking_history_json(&key).unwrap_or_default())
    }

    #[napi]
    pub async fn autopilot_set_controllers(&self, json: String) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
            svc.set_controllers(&json);
            Ok(())
    }

    #[napi]
    pub async fn autopilot_set_current_controller(&self, json: String) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
            svc.set_current_controller(&json);
            Ok(())
    }

    #[napi]
    pub async fn autopilot_add_controller(&self, json: String) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
            svc.add_controller(&json);
            Ok(())
    }

    #[napi]
    pub async fn autopilot_update_controller(&self, key: String, json: String) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
            svc.update_controller(&key, &json);
            Ok(())
    }

    #[napi]
    pub async fn autopilot_remove_controller(&self, key: String) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
            svc.remove_controller(&key);
            Ok(())
    }

    #[napi]
    pub async fn autopilot_set_iterations(&self, key: String, json: String) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
            svc.set_iterations(&key, &json);
            Ok(())
    }

    #[napi]
    pub async fn autopilot_add_iteration(&self, key: String, json: String) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
            svc.add_iteration(&key, &json);
            Ok(())
    }

    #[napi]
    pub async fn autopilot_update_thinking(&self, key: String, json: String) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
            svc.update_thinking(&key, &json);
            Ok(())
    }

    // ===== autopilot_api.rs =====
    #[napi]
    pub async fn autopilot_fetch_controllers(&self) -> napi::Result<String> {
        let svc = self.autopilot.lock().await;
            svc.fetch_controllers().await.map_err(err)
    }

    #[napi]
    pub async fn autopilot_fetch_controller(&self, key: String) -> napi::Result<String> {
        let svc = self.autopilot.lock().await;
            svc.fetch_controller(&key).await.map_err(err)
    }

    #[napi]
    pub async fn autopilot_create_controller(&self, request_json: String) -> napi::Result<String> {
        let svc = self.autopilot.lock().await;
            svc.create_controller(&request_json).await.map_err(err)
    }

    #[napi]
    pub async fn autopilot_pause_controller(&self, key: String) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
            svc.pause_controller(&key).await.map_err(err)
    }

    #[napi]
    pub async fn autopilot_resume_controller(&self, key: String) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
            svc.resume_controller(&key).await.map_err(err)
    }

    #[napi]
    pub async fn autopilot_stop_controller(&self, key: String) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
            svc.stop_controller(&key).await.map_err(err)
    }

    #[napi]
    pub async fn autopilot_approve_controller(&self, key: String, request_json: String) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
            svc.approve_controller(&key, &request_json).await.map_err(err)
    }

    #[napi]
    pub async fn autopilot_takeover_controller(&self, key: String) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
            svc.takeover_controller(&key).await.map_err(err)
    }

    #[napi]
    pub async fn autopilot_handback_controller(&self, key: String) -> napi::Result<()> {
        let svc = self.autopilot.lock().await;
            svc.handback_controller(&key).await.map_err(err)
    }

    #[napi]
    pub async fn autopilot_fetch_iterations(&self, key: String) -> napi::Result<String> {
        let svc = self.autopilot.lock().await;
            svc.fetch_iterations(&key).await.map_err(err)
    }

    // ===== billing.rs =====
    #[napi]
    pub async fn billing_get_overview(&self) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.get_overview().await.map_err(err)
    }

    #[napi]
    pub async fn billing_get_subscription(&self) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.get_subscription().await.map_err(err)
    }

    #[napi]
    pub async fn billing_create_subscription(&self, json: String) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.create_subscription(&json).await.map_err(err)
    }

    #[napi]
    pub async fn billing_cancel_subscription(&self) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.cancel_subscription().await.map_err(err)
    }

    #[napi]
    pub async fn billing_update_subscription(&self, json: String) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.update_subscription(&json).await.map_err(err)
    }

    #[napi]
    pub async fn billing_list_plans(&self) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.list_plans().await.map_err(err)
    }

    #[napi]
    pub async fn billing_get_usage(&self, usage_type: Option<String>) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.get_usage(usage_type).await.map_err(err)
    }

    #[napi]
    pub async fn billing_check_quota(&self, resource: String, amount: Option<u32>) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.check_quota(&resource, amount).await.map_err(err)
    }

    #[napi]
    pub async fn billing_create_checkout(&self, json: String) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.create_checkout(&json).await.map_err(err)
    }

    #[napi]
    pub async fn billing_get_checkout_status(&self, order_no: String) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.get_checkout_status(&order_no).await.map_err(err)
    }

    #[napi]
    pub async fn billing_request_cancel(&self, json: String) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.request_cancel(&json).await.map_err(err)
    }

    #[napi]
    pub async fn billing_reactivate(&self) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.reactivate().await.map_err(err)
    }

    #[napi]
    pub async fn billing_upgrade(&self, json: String) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.upgrade(&json).await.map_err(err)
    }

    #[napi]
    pub async fn billing_change_cycle(&self, json: String) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.change_cycle(&json).await.map_err(err)
    }

    #[napi]
    pub async fn billing_update_auto_renew(&self, json: String) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.update_auto_renew(&json).await.map_err(err)
    }

    #[napi]
    pub async fn billing_get_seat_usage(&self) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.get_seat_usage().await.map_err(err)
    }

    #[napi]
    pub async fn billing_purchase_seats(&self, json: String) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.purchase_seats(&json).await.map_err(err)
    }

    #[napi]
    pub async fn billing_list_invoices(&self, limit: Option<u32>, offset: Option<u32>) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.list_invoices(limit, offset).await.map_err(err)
    }

    #[napi]
    pub async fn billing_get_customer_portal(&self, json: String) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.get_customer_portal(&json).await.map_err(err)
    }

    #[napi]
    pub async fn billing_get_deployment_info(&self) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.get_deployment_info().await.map_err(err)
    }

    #[napi]
    pub async fn billing_get_public_pricing(&self) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.get_public_pricing().await.map_err(err)
    }

    #[napi]
    pub async fn billing_get_public_deployment_info(&self) -> napi::Result<String> {
        let svc = self.billing.lock().await;
            svc.get_public_deployment_info().await.map_err(err)
    }

    // ===== binding.rs =====
    #[napi]
    pub async fn binding_request_binding(&self, json: String, pod_key: Option<String>) -> napi::Result<String> {
        let svc = self.binding.lock().await;
            svc.request_binding(&json, pod_key).await.map_err(err)
    }

    #[napi]
    pub async fn binding_accept_binding(&self, json: String) -> napi::Result<String> {
        let svc = self.binding.lock().await;
            svc.accept_binding(&json).await.map_err(err)
    }

    #[napi]
    pub async fn binding_reject_binding(&self, json: String) -> napi::Result<()> {
        let svc = self.binding.lock().await;
            svc.reject_binding(&json).await.map_err(err)
    }

    #[napi]
    pub async fn binding_request_scopes(&self, binding_id: i64, json: String) -> napi::Result<String> {
        let svc = self.binding.lock().await;
            svc.request_scopes(binding_id, &json).await.map_err(err)
    }

    #[napi]
    pub async fn binding_approve_scopes(&self, binding_id: i64, json: String) -> napi::Result<String> {
        let svc = self.binding.lock().await;
            svc.approve_scopes(binding_id, &json).await.map_err(err)
    }

    #[napi]
    pub async fn binding_unbind(&self, json: String) -> napi::Result<()> {
        let svc = self.binding.lock().await;
            svc.unbind(&json).await.map_err(err)
    }

    #[napi]
    pub async fn binding_list_bindings(&self, status: Option<String>) -> napi::Result<String> {
        let svc = self.binding.lock().await;
            svc.list_bindings(status).await.map_err(err)
    }

    #[napi]
    pub async fn binding_get_pending_bindings(&self) -> napi::Result<String> {
        let svc = self.binding.lock().await;
            svc.get_pending_bindings().await.map_err(err)
    }

    #[napi]
    pub async fn binding_get_bound_pods(&self) -> napi::Result<String> {
        let svc = self.binding.lock().await;
            svc.get_bound_pods().await.map_err(err)
    }

    #[napi]
    pub async fn binding_check_binding(&self, target_pod: String) -> napi::Result<String> {
        let svc = self.binding.lock().await;
            svc.check_binding(&target_pod).await.map_err(err)
    }

    // ===== channel.rs =====
    #[napi]
    pub async fn channel_channels_json(&self) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            Ok(svc.channels_json())
    }

    #[napi]
    pub async fn channel_current_channel_json(&self) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            Ok(svc.current_channel_json().unwrap_or_default())
    }

    #[napi]
    pub async fn channel_get_channel_json(&self, id: i64) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            Ok(svc.get_channel_json(id).unwrap_or_default())
    }

    #[napi]
    pub async fn channel_filter_channels_json(&self, query: String, include_archived: bool) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            Ok(svc.filter_channels_json(&query, include_archived))
    }

    #[napi]
    pub async fn channel_get_messages_json(&self, channel_id: i64) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            Ok(svc.get_messages_json(channel_id).unwrap_or_default())
    }

    #[napi]
    pub async fn channel_get_unread_count(&self, channel_id: i64) -> napi::Result<u32> {
        let svc = self.channel.lock().await;
            Ok(svc.get_unread_count(channel_id))
    }

    #[napi]
    pub async fn channel_total_unread_count(&self) -> napi::Result<u32> {
        let svc = self.channel.lock().await;
            Ok(svc.total_unread_count())
    }

    #[napi]
    pub async fn channel_unread_counts_json(&self) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            Ok(svc.unread_counts_json())
    }

    #[napi]
    pub async fn channel_get_mention_count(&self, channel_id: i64) -> napi::Result<u32> {
        let svc = self.channel.lock().await;
            Ok(svc.get_mention_count(channel_id))
    }

    #[napi]
    pub async fn channel_total_mention_count(&self) -> napi::Result<u32> {
        let svc = self.channel.lock().await;
            Ok(svc.total_mention_count())
    }

    #[napi]
    pub async fn channel_mention_counts_json(&self) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            Ok(svc.mention_counts_json())
    }

    #[napi]
    pub async fn channel_sorted_channel_ids_json(&self, mode: String, include_archived: bool) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            Ok(svc.sorted_channel_ids_json(&mode, include_archived))
    }

    #[napi]
    pub async fn channel_get_last_message_json(&self, channel_id: i64) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            Ok(svc.get_last_message_json(channel_id).unwrap_or_default())
    }

    #[napi]
    pub async fn channel_set_channels(&self, json: String) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.set_channels(&json);
            Ok(())
    }

    #[napi]
    pub async fn channel_set_current_channel(&self, id: Option<i64>) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.set_current_channel(id);
            Ok(())
    }

    #[napi]
    pub async fn channel_select_channel(&self, id: Option<i64>) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            Ok(svc.select_channel(id).unwrap_or_default())
    }

    #[napi]
    pub async fn channel_add_channel_local(&self, json: String) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.add_channel_local(&json);
            Ok(())
    }

    #[napi]
    pub async fn channel_update_channel_local(&self, id: i64, json: String) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.update_channel_local(id, &json);
            Ok(())
    }

    #[napi]
    pub async fn channel_remove_channel_local(&self, id: i64) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.remove_channel_local(id);
            Ok(())
    }

    // ===== channel_api.rs =====
    #[napi]
    pub async fn channel_fetch_channels(&self, include_archived: Option<bool>) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            svc.fetch_channels(include_archived).await.map_err(err)
    }

    #[napi]
    pub async fn channel_fetch_channel(&self, id: i64) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            svc.fetch_channel(id).await.map_err(err)
    }

    #[napi]
    pub async fn channel_create_channel(&self, request_json: String) -> napi::Result<String> {
        let svc = self.channel.lock().await;
        svc.create_channel(&request_json).await.map_err(err)
    }

    #[napi]
    pub async fn channel_update_channel(&self, id: i64, request_json: String) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            svc.update_channel(id, &request_json).await.map_err(err)
    }

    #[napi]
    pub async fn channel_archive_channel(&self, id: i64) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.archive_channel(id).await.map_err(err)
    }

    #[napi]
    pub async fn channel_unarchive_channel(&self, id: i64) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.unarchive_channel(id).await.map_err(err)
    }

    #[napi]
    pub async fn channel_join_channel(&self, channel_id: i64, pod_key: String) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            svc.join_channel(channel_id, &pod_key).await.map_err(err)
    }

    #[napi]
    pub async fn channel_leave_channel(&self, channel_id: i64, pod_key: String) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            svc.leave_channel(channel_id, &pod_key).await.map_err(err)
    }

    #[napi]
    pub async fn channel_fetch_messages(&self, channel_id: i64, limit: Option<u32>, before_id: Option<i64>) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            svc.fetch_messages(channel_id, limit, before_id).await.map_err(err)
    }

    #[napi]
    pub async fn channel_send_message(&self, channel_id: i64, request_json: String) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            svc.send_message(channel_id, &request_json).await.map_err(err)
    }

    #[napi]
    pub async fn channel_edit_message(&self, channel_id: i64, message_id: i64, content: String) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            svc.edit_message(channel_id, message_id, &content).await.map_err(err)
    }

    #[napi]
    pub async fn channel_delete_message(&self, channel_id: i64, message_id: i64) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.delete_message(channel_id, message_id).await.map_err(err)
    }

    #[napi]
    pub async fn channel_fetch_unread_counts(&self) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            svc.fetch_unread_counts().await.map_err(err)
    }

    #[napi]
    pub async fn channel_mark_read(&self, channel_id: i64, message_id: i64) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.mark_read(channel_id, message_id).await.map_err(err)
    }

    #[napi]
    pub async fn channel_mute_channel(&self, channel_id: i64, muted: bool) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.mute_channel(channel_id, muted).await.map_err(err)
    }

    #[napi]
    pub async fn channel_get_channel_pods(&self, id: i64) -> napi::Result<String> {
        let svc = self.channel.lock().await;
            svc.get_channel_pods(id).await.map_err(err)
    }

    // ===== channel_state.rs =====
    #[napi]
    pub async fn channel_set_current_user(&self, user_json: String) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.set_current_user(&user_json);
            Ok(())
    }

    #[napi]
    pub async fn channel_set_current_user_id(&self, user_id: Option<i64>) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.set_current_user_id(user_id);
            Ok(())
    }

    #[napi]
    pub async fn channel_set_messages(&self, channel_id: i64, json: String, has_more: bool) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.set_messages(channel_id, &json, has_more);
            Ok(())
    }

    #[napi]
    pub async fn channel_prepend_messages(&self, channel_id: i64, json: String, has_more: bool) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.prepend_messages(channel_id, &json, has_more);
            Ok(())
    }

    #[napi]
    pub async fn channel_add_message(&self, channel_id: i64, json: String) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.add_message(channel_id, &json);
            Ok(())
    }

    #[napi]
    pub async fn channel_on_new_message(&self, json: String) -> napi::Result<bool> {
        let svc = self.channel.lock().await;
            Ok(svc.on_new_message(&json))
    }

    #[napi]
    pub async fn channel_update_message_local(&self, channel_id: i64, json: String) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.update_message_local(channel_id, &json);
            Ok(())
    }

    #[napi]
    pub async fn channel_remove_message_local(&self, channel_id: i64, message_id: i64) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.remove_message_local(channel_id, message_id);
            Ok(())
    }

    #[napi]
    pub async fn channel_set_unread_counts(&self, json: String) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.set_unread_counts(&json);
            Ok(())
    }

    #[napi]
    pub async fn channel_increment_unread(&self, channel_id: i64) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.increment_unread(channel_id);
            Ok(())
    }

    #[napi]
    pub async fn channel_clear_channel_unread(&self, channel_id: i64) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.clear_channel_unread(channel_id);
            Ok(())
    }

    #[napi]
    pub async fn channel_set_mention_counts(&self, json: String) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.set_mention_counts(&json);
            Ok(())
    }

    #[napi]
    pub async fn channel_increment_mention(&self, channel_id: i64) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.increment_mention(channel_id);
            Ok(())
    }

    #[napi]
    pub async fn channel_clear_channel_mentions(&self, channel_id: i64) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.clear_channel_mentions(channel_id);
            Ok(())
    }

    #[napi]
    pub async fn channel_set_last_message(&self, channel_id: i64, json: String) -> napi::Result<()> {
        let svc = self.channel.lock().await;
            svc.set_last_message(channel_id, &json);
            Ok(())
    }

    // ===== extension.rs =====
    #[napi]
    pub async fn extension_list_skill_registries(&self) -> napi::Result<String> {
        let svc = self.extension.lock().await;
            svc.list_skill_registries().await.map_err(err)
    }

    #[napi]
    pub async fn extension_create_skill_registry(&self, json: String) -> napi::Result<String> {
        let svc = self.extension.lock().await;
            svc.create_skill_registry(&json).await.map_err(err)
    }

    #[napi]
    pub async fn extension_sync_skill_registry(&self, id: i64) -> napi::Result<()> {
        let svc = self.extension.lock().await;
            svc.sync_skill_registry(id).await.map_err(err)
    }

    #[napi]
    pub async fn extension_toggle_skill_registry(&self, id: i64, json: String) -> napi::Result<String> {
        let svc = self.extension.lock().await;
            svc.toggle_skill_registry(id, &json).await.map_err(err)
    }

    #[napi]
    pub async fn extension_delete_skill_registry(&self, id: i64) -> napi::Result<()> {
        let svc = self.extension.lock().await;
            svc.delete_skill_registry(id).await.map_err(err)
    }

    #[napi]
    pub async fn extension_list_skill_registry_overrides(&self) -> napi::Result<String> {
        let svc = self.extension.lock().await;
            svc.list_skill_registry_overrides().await.map_err(err)
    }

    #[napi]
    pub async fn extension_list_market_skills(&self, query: Option<String>, category: Option<String>) -> napi::Result<String> {
        let svc = self.extension.lock().await;
            svc.list_market_skills(query, category).await.map_err(err)
    }

    #[napi]
    pub async fn extension_list_market_mcp_servers(&self, query: Option<String>, limit: Option<u32>, offset: Option<u32>) -> napi::Result<String> {
        let svc = self.extension.lock().await;
            svc.list_market_mcp_servers(query, limit, offset).await.map_err(err)
    }

    #[napi]
    pub async fn extension_list_repo_skills(&self, repo_id: i64, scope: Option<String>) -> napi::Result<String> {
        let svc = self.extension.lock().await;
            svc.list_repo_skills(repo_id, scope).await.map_err(err)
    }

    #[napi]
    pub async fn extension_install_skill_from_market(&self, repo_id: i64, json: String) -> napi::Result<String> {
        let svc = self.extension.lock().await;
            svc.install_skill_from_market(repo_id, &json).await.map_err(err)
    }

    #[napi]
    pub async fn extension_install_skill_from_github(&self, repo_id: i64, json: String) -> napi::Result<String> {
        let svc = self.extension.lock().await;
            svc.install_skill_from_github(repo_id, &json).await.map_err(err)
    }

    #[napi]
    pub async fn extension_update_skill(&self, repo_id: i64, install_id: i64, json: String) -> napi::Result<String> {
        let svc = self.extension.lock().await;
            svc.update_skill(repo_id, install_id, &json).await.map_err(err)
    }

    #[napi]
    pub async fn extension_uninstall_skill(&self, repo_id: i64, install_id: i64) -> napi::Result<()> {
        let svc = self.extension.lock().await;
            svc.uninstall_skill(repo_id, install_id).await.map_err(err)
    }

    #[napi]
    pub async fn extension_list_repo_mcp_servers(&self, repo_id: i64, scope: Option<String>) -> napi::Result<String> {
        let svc = self.extension.lock().await;
            svc.list_repo_mcp_servers(repo_id, scope).await.map_err(err)
    }

    #[napi]
    pub async fn extension_install_mcp_from_market(&self, repo_id: i64, json: String) -> napi::Result<String> {
        let svc = self.extension.lock().await;
            svc.install_mcp_from_market(repo_id, &json).await.map_err(err)
    }

    #[napi]
    pub async fn extension_install_custom_mcp_server(&self, repo_id: i64, json: String) -> napi::Result<String> {
        let svc = self.extension.lock().await;
            svc.install_custom_mcp_server(repo_id, &json).await.map_err(err)
    }

    #[napi]
    pub async fn extension_update_mcp_server(&self, repo_id: i64, install_id: i64, json: String) -> napi::Result<String> {
        let svc = self.extension.lock().await;
            svc.update_mcp_server(repo_id, install_id, &json).await.map_err(err)
    }

    #[napi]
    pub async fn extension_uninstall_mcp_server(&self, repo_id: i64, install_id: i64) -> napi::Result<()> {
        let svc = self.extension.lock().await;
            svc.uninstall_mcp_server(repo_id, install_id).await.map_err(err)
    }

    #[napi]
    pub async fn extension_install_skill_from_upload(&self, repo_id: i64, file_data: Vec<u8>, file_name: String, scope: Option<String>) -> napi::Result<String> {
        let svc = self.extension.lock().await;
            svc.install_skill_from_upload(repo_id, file_data, &file_name, scope).await.map_err(err)
    }

    // ===== file.rs =====
    #[napi]
    pub async fn file_presign_upload(&self, json: String) -> napi::Result<String> {
        let svc = self.file.lock().await;
            svc.presign_upload(&json).await.map_err(err)
    }

    #[napi]
    pub async fn file_upload_file(&self, file_data: Vec<u8>, filename: String, content_type: String) -> napi::Result<String> {
        let svc = self.file.lock().await;
            svc.upload_file(file_data, &filename, &content_type).await.map_err(err)
    }

    // ===== invitation.rs =====
    #[napi]
    pub async fn invitation_list(&self) -> napi::Result<String> {
        let svc = self.invitation.lock().await;
            svc.list().await.map_err(err)
    }

    #[napi]
    pub async fn invitation_create(&self, json: String) -> napi::Result<String> {
        let svc = self.invitation.lock().await;
            svc.create(&json).await.map_err(err)
    }

    #[napi]
    pub async fn invitation_revoke(&self, id: i64) -> napi::Result<()> {
        let svc = self.invitation.lock().await;
            svc.revoke(id).await.map_err(err)
    }

    #[napi]
    pub async fn invitation_resend(&self, id: i64) -> napi::Result<()> {
        let svc = self.invitation.lock().await;
            svc.resend(id).await.map_err(err)
    }

    #[napi]
    pub async fn invitation_get_by_token(&self, token: String) -> napi::Result<String> {
        let svc = self.invitation.lock().await;
            svc.get_by_token(&token).await.map_err(err)
    }

    #[napi]
    pub async fn invitation_accept(&self, token: String) -> napi::Result<()> {
        let svc = self.invitation.lock().await;
            svc.accept(&token).await.map_err(err)
    }

    #[napi]
    pub async fn invitation_list_pending(&self) -> napi::Result<String> {
        let svc = self.invitation.lock().await;
            svc.list_pending().await.map_err(err)
    }

    // ===== loop_service.rs =====
    #[napi]
    pub async fn loop_svc_loops_json(&self) -> napi::Result<String> {
        let svc = self.loop_svc.lock().await;
            Ok(svc.loops_json())
    }

    #[napi]
    pub async fn loop_svc_current_loop_json(&self) -> napi::Result<String> {
        let svc = self.loop_svc.lock().await;
            Ok(svc.current_loop_json().unwrap_or_default())
    }

    #[napi]
    pub async fn loop_svc_runs_json(&self) -> napi::Result<String> {
        let svc = self.loop_svc.lock().await;
            Ok(svc.runs_json())
    }

    #[napi]
    pub async fn loop_svc_get_loop_by_slug_json(&self, slug: String) -> napi::Result<String> {
        let svc = self.loop_svc.lock().await;
            Ok(svc.get_loop_by_slug_json(&slug).unwrap_or_default())
    }

    #[napi]
    pub async fn loop_svc_set_loops(&self, json: String) -> napi::Result<()> {
        let svc = self.loop_svc.lock().await;
            svc.set_loops(&json);
            Ok(())
    }

    #[napi]
    pub async fn loop_svc_set_current_loop(&self, json: String) -> napi::Result<()> {
        let svc = self.loop_svc.lock().await;
            svc.set_current_loop(&json);
            Ok(())
    }

    #[napi]
    pub async fn loop_svc_update_loop_local(&self, slug: String, json: String) -> napi::Result<()> {
        let svc = self.loop_svc.lock().await;
            svc.update_loop_local(&slug, &json);
            Ok(())
    }

    #[napi]
    pub async fn loop_svc_add_run(&self, json: String) -> napi::Result<()> {
        let svc = self.loop_svc.lock().await;
            svc.add_run(&json);
            Ok(())
    }

    #[napi]
    pub async fn loop_svc_set_runs(&self, json: String) -> napi::Result<()> {
        let svc = self.loop_svc.lock().await;
            svc.set_runs(&json);
            Ok(())
    }

    #[napi]
    pub async fn loop_svc_append_runs(&self, json: String) -> napi::Result<()> {
        let svc = self.loop_svc.lock().await;
            svc.append_runs(&json);
            Ok(())
    }

    #[napi]
    pub async fn loop_svc_update_run_status(&self, run_id: i64, status: String) -> napi::Result<()> {
        let svc = self.loop_svc.lock().await;
            svc.update_run_status(run_id, &status);
            Ok(())
    }

    #[napi]
    pub async fn loop_svc_clear_runs(&self) -> napi::Result<()> {
        let svc = self.loop_svc.lock().await;
            svc.clear_runs();
            Ok(())
    }

    #[napi]
    pub async fn loop_svc_fetch_loops(&self, status: Option<String>, limit: Option<u32>, offset: Option<u32>) -> napi::Result<String> {
        let svc = self.loop_svc.lock().await;
            svc.fetch_loops(status, limit, offset).await.map_err(err)
    }

    #[napi]
    pub async fn loop_svc_fetch_loop(&self, slug: String) -> napi::Result<String> {
        let svc = self.loop_svc.lock().await;
            svc.fetch_loop(&slug).await.map_err(err)
    }

    #[napi]
    pub async fn loop_svc_create_loop(&self, request_json: String) -> napi::Result<String> {
        let svc = self.loop_svc.lock().await;
            svc.create_loop(&request_json).await.map_err(err)
    }

    #[napi]
    pub async fn loop_svc_update_loop(&self, slug: String, request_json: String) -> napi::Result<String> {
        let svc = self.loop_svc.lock().await;
            svc.update_loop(&slug, &request_json).await.map_err(err)
    }

    #[napi]
    pub async fn loop_svc_delete_loop(&self, slug: String) -> napi::Result<()> {
        let svc = self.loop_svc.lock().await;
            svc.delete_loop(&slug).await.map_err(err)
    }

    #[napi]
    pub async fn loop_svc_enable_loop(&self, slug: String) -> napi::Result<String> {
        let svc = self.loop_svc.lock().await;
            svc.enable_loop(&slug).await.map_err(err)
    }

    #[napi]
    pub async fn loop_svc_disable_loop(&self, slug: String) -> napi::Result<String> {
        let svc = self.loop_svc.lock().await;
            svc.disable_loop(&slug).await.map_err(err)
    }

    #[napi]
    pub async fn loop_svc_trigger_loop(&self, slug: String) -> napi::Result<String> {
        let svc = self.loop_svc.lock().await;
            svc.trigger_loop(&slug).await.map_err(err)
    }

    #[napi]
    pub async fn loop_svc_fetch_runs(&self, slug: String, status: Option<String>, limit: Option<u32>, offset: Option<u32>) -> napi::Result<String> {
        let svc = self.loop_svc.lock().await;
            svc.fetch_runs(&slug, status, limit, offset).await.map_err(err)
    }

    #[napi]
    pub async fn loop_svc_cancel_run(&self, slug: String, run_id: i64) -> napi::Result<()> {
        let svc = self.loop_svc.lock().await;
            svc.cancel_run(&slug, run_id).await.map_err(err)
    }

    // ===== mesh.rs =====
    #[napi]
    pub async fn mesh_topology_json(&self) -> napi::Result<String> {
        let svc = self.mesh.lock().await;
            Ok(svc.topology_json().unwrap_or_default())
    }

    #[napi]
    pub async fn mesh_selected_node(&self) -> napi::Result<String> {
        let svc = self.mesh.lock().await;
            Ok(svc.selected_node().unwrap_or_default())
    }

    #[napi]
    pub async fn mesh_get_node_json(&self, pod_key: String) -> napi::Result<String> {
        let svc = self.mesh.lock().await;
            Ok(svc.get_node_json(&pod_key).unwrap_or_default())
    }

    #[napi]
    pub async fn mesh_get_edges_for_node_json(&self, pod_key: String) -> napi::Result<String> {
        let svc = self.mesh.lock().await;
            Ok(svc.get_edges_for_node_json(&pod_key))
    }

    #[napi]
    pub async fn mesh_get_channels_for_node_json(&self, pod_key: String) -> napi::Result<String> {
        let svc = self.mesh.lock().await;
            Ok(svc.get_channels_for_node_json(&pod_key))
    }

    #[napi]
    pub async fn mesh_get_active_nodes_json(&self) -> napi::Result<String> {
        let svc = self.mesh.lock().await;
            Ok(svc.get_active_nodes_json())
    }

    #[napi]
    pub async fn mesh_get_nodes_by_runner_json(&self, runner_id: i64) -> napi::Result<String> {
        let svc = self.mesh.lock().await;
            Ok(svc.get_nodes_by_runner_json(runner_id))
    }

    #[napi]
    pub async fn mesh_get_runner_info_json(&self, runner_id: i64) -> napi::Result<String> {
        let svc = self.mesh.lock().await;
            Ok(svc.get_runner_info_json(runner_id).unwrap_or_default())
    }

    #[napi]
    pub async fn mesh_set_topology(&self, json: String) -> napi::Result<()> {
        let svc = self.mesh.lock().await;
            svc.set_topology(&json);
            Ok(())
    }

    #[napi]
    pub async fn mesh_clear_topology(&self) -> napi::Result<()> {
        let svc = self.mesh.lock().await;
            svc.clear_topology();
            Ok(())
    }

    #[napi]
    pub async fn mesh_select_node(&self, pod_key: Option<String>) -> napi::Result<()> {
        let svc = self.mesh.lock().await;
            svc.select_node(pod_key);
            Ok(())
    }

    #[napi]
    pub async fn mesh_fetch_topology(&self) -> napi::Result<String> {
        let svc = self.mesh.lock().await;
            svc.fetch_topology().await.map_err(err)
    }

    // ===== message.rs =====
    #[napi]
    pub async fn message_send_message(&self, json: String, pod_key: Option<String>) -> napi::Result<String> {
        let svc = self.message.lock().await;
            svc.send_message(&json, pod_key).await.map_err(err)
    }

    #[napi]
    pub async fn message_get_messages(&self, unread_only: Option<bool>, limit: Option<u32>, offset: Option<u32>) -> napi::Result<String> {
        let svc = self.message.lock().await;
            svc.get_messages(unread_only, limit, offset).await.map_err(err)
    }

    #[napi]
    pub async fn message_get_unread_count(&self) -> napi::Result<String> {
        let svc = self.message.lock().await;
            svc.get_unread_count().await.map_err(err)
    }

    #[napi]
    pub async fn message_get_message(&self, id: i64) -> napi::Result<String> {
        let svc = self.message.lock().await;
            svc.get_message(id).await.map_err(err)
    }

    #[napi]
    pub async fn message_mark_read(&self, json: String) -> napi::Result<()> {
        let svc = self.message.lock().await;
            svc.mark_read(&json).await.map_err(err)
    }

    #[napi]
    pub async fn message_mark_all_read(&self) -> napi::Result<()> {
        let svc = self.message.lock().await;
            svc.mark_all_read().await.map_err(err)
    }

    #[napi]
    pub async fn message_get_conversation(&self, correlation_id: String, limit: Option<u32>) -> napi::Result<String> {
        let svc = self.message.lock().await;
            svc.get_conversation(&correlation_id, limit).await.map_err(err)
    }

    #[napi]
    pub async fn message_get_sent_messages(&self, limit: Option<u32>, offset: Option<u32>) -> napi::Result<String> {
        let svc = self.message.lock().await;
            svc.get_sent_messages(limit, offset).await.map_err(err)
    }

    #[napi]
    pub async fn message_get_dead_letters(&self, limit: Option<u32>, offset: Option<u32>) -> napi::Result<String> {
        let svc = self.message.lock().await;
            svc.get_dead_letters(limit, offset).await.map_err(err)
    }

    #[napi]
    pub async fn message_replay_dead_letter(&self, entry_id: i64) -> napi::Result<()> {
        let svc = self.message.lock().await;
            svc.replay_dead_letter(entry_id).await.map_err(err)
    }

    // ===== notification.rs =====
    #[napi]
    pub async fn notification_get_preferences(&self) -> napi::Result<String> {
        let svc = self.notification.lock().await;
            svc.get_preferences().await.map_err(err)
    }

    #[napi]
    pub async fn notification_set_preference(&self, json: String) -> napi::Result<String> {
        let svc = self.notification.lock().await;
            svc.set_preference(&json).await.map_err(err)
    }

    // ===== org.rs =====
    #[napi]
    pub async fn org_list(&self) -> napi::Result<String> {
        let svc = self.org.lock().await;
            svc.list().await.map_err(err)
    }

    #[napi]
    pub async fn org_get(&self, slug: String) -> napi::Result<String> {
        let svc = self.org.lock().await;
            svc.get(&slug).await.map_err(err)
    }

    #[napi]
    pub async fn org_create(&self, json: String) -> napi::Result<String> {
        let svc = self.org.lock().await;
            svc.create(&json).await.map_err(err)
    }

    #[napi]
    pub async fn org_update(&self, slug: String, json: String) -> napi::Result<String> {
        let svc = self.org.lock().await;
            svc.update(&slug, &json).await.map_err(err)
    }

    #[napi]
    pub async fn org_delete(&self, slug: String) -> napi::Result<()> {
        let svc = self.org.lock().await;
            svc.delete(&slug).await.map_err(err)
    }

    #[napi]
    pub async fn org_list_members(&self, slug: String) -> napi::Result<String> {
        let svc = self.org.lock().await;
            svc.list_members(&slug).await.map_err(err)
    }

    #[napi]
    pub async fn org_invite_member(&self, slug: String, json: String) -> napi::Result<String> {
        let svc = self.org.lock().await;
            svc.invite_member(&slug, &json).await.map_err(err)
    }

    #[napi]
    pub async fn org_remove_member(&self, slug: String, user_id: i64) -> napi::Result<()> {
        let svc = self.org.lock().await;
            svc.remove_member(&slug, user_id).await.map_err(err)
    }

    #[napi]
    pub async fn org_update_member_role(&self, slug: String, user_id: i64, json: String) -> napi::Result<String> {
        let svc = self.org.lock().await;
            svc.update_member_role(&slug, user_id, &json).await.map_err(err)
    }

    // ===== pod.rs =====
    #[napi]
    pub async fn pod_pods_json(&self) -> napi::Result<String> {
        let svc = self.pod.lock().await;
            Ok(svc.pods_json())
    }

    #[napi]
    pub async fn pod_current_pod_json(&self) -> napi::Result<String> {
        let svc = self.pod.lock().await;
            Ok(svc.current_pod_json().unwrap_or_default())
    }

    #[napi]
    pub async fn pod_get_pod_json(&self, pod_key: String) -> napi::Result<String> {
        let svc = self.pod.lock().await;
            Ok(svc.get_pod_json(&pod_key).unwrap_or_default())
    }

    #[napi]
    pub async fn pod_upsert_pod(&self, pod_json: String, timestamp: Option<i64>) -> napi::Result<()> {
        let svc = self.pod.lock().await;
            svc.upsert_pod(&pod_json, timestamp);
            Ok(())
    }

    #[napi]
    pub async fn pod_set_pods(&self, pods_json: String) -> napi::Result<()> {
        let svc = self.pod.lock().await;
            svc.set_pods(&pods_json);
            Ok(())
    }

    #[napi]
    pub async fn pod_set_current_pod(&self, pod_json: String) -> napi::Result<()> {
        let svc = self.pod.lock().await;
            svc.set_current_pod(&pod_json);
            Ok(())
    }

    #[napi]
    pub async fn pod_update_pod_status(&self, pod_key: String, status: String, agent_status: Option<String>, error_code: Option<String>, error_message: Option<String>, timestamp: Option<i64>) -> napi::Result<()> {
        let svc = self.pod.lock().await;
            svc.update_pod_status(&pod_key, &status, agent_status, error_code, error_message, timestamp);
            Ok(())
    }

    #[napi]
    pub async fn pod_update_pod_title(&self, pod_key: String, title: String, timestamp: Option<i64>) -> napi::Result<()> {
        let svc = self.pod.lock().await;
            svc.update_pod_title(&pod_key, &title, timestamp);
            Ok(())
    }

    #[napi]
    pub async fn pod_update_pod_alias(&self, pod_key: String, alias: String) -> napi::Result<()> {
        let svc = self.pod.lock().await;
            svc.update_pod_alias(&pod_key, &alias);
            Ok(())
    }

    #[napi]
    pub async fn pod_update_agent_status(&self, pod_key: String, agent_status: String) -> napi::Result<()> {
        let svc = self.pod.lock().await;
            svc.update_agent_status(&pod_key, &agent_status);
            Ok(())
    }

    #[napi]
    pub async fn pod_remove_pod(&self, pod_key: String) -> napi::Result<()> {
        let svc = self.pod.lock().await;
            svc.remove_pod(&pod_key);
            Ok(())
    }

    #[napi]
    pub async fn pod_fetch_pods(&self, status: Option<String>, runner_id: Option<i64>, created_by_id: Option<i64>, limit: Option<i64>, offset: Option<i64>) -> napi::Result<String> {
        let svc = self.pod.lock().await;
            svc.fetch_pods(status, runner_id, created_by_id, limit, offset).await.map_err(err)
    }

    #[napi]
    pub async fn pod_fetch_sidebar_pods(&self, filter: String, user_id: Option<i64>) -> napi::Result<String> {
        let svc = self.pod.lock().await;
            svc.fetch_sidebar_pods(&filter, user_id).await.map_err(err)
    }

    #[napi]
    pub async fn pod_load_more_pods(&self, filter: String, user_id: Option<i64>, offset: i64) -> napi::Result<String> {
        let svc = self.pod.lock().await;
            svc.load_more_pods(&filter, user_id, offset).await.map_err(err)
    }

    #[napi]
    pub async fn pod_fetch_pod(&self, pod_key: String) -> napi::Result<String> {
        let svc = self.pod.lock().await;
            svc.fetch_pod(&pod_key).await.map_err(err)
    }

    #[napi]
    pub async fn pod_create_pod(&self, request_json: String) -> napi::Result<String> {
        let svc = self.pod.lock().await;
            svc.create_pod(&request_json).await.map_err(err)
    }

    #[napi]
    pub async fn pod_terminate_pod(&self, pod_key: String) -> napi::Result<()> {
        let svc = self.pod.lock().await;
            svc.terminate_pod(&pod_key).await.map_err(err)
    }

    #[napi]
    pub async fn pod_update_pod_alias_api(&self, pod_key: String, alias: Option<String>) -> napi::Result<()> {
        let svc = self.pod.lock().await;
            svc.update_pod_alias_api(&pod_key, alias).await.map_err(err)
    }

    #[napi]
    pub async fn pod_get_pod_connection(&self, pod_key: String) -> napi::Result<String> {
        let svc = self.pod.lock().await;
            svc.get_pod_connection(&pod_key).await.map_err(err)
    }

    // ===== repository.rs =====
    #[napi]
    pub async fn repository_list(&self) -> napi::Result<String> {
        let svc = self.repository.lock().await;
            svc.list().await.map_err(err)
    }

    #[napi]
    pub async fn repository_get(&self, id: i64) -> napi::Result<String> {
        let svc = self.repository.lock().await;
            svc.get(id).await.map_err(err)
    }

    #[napi]
    pub async fn repository_create(&self, json: String) -> napi::Result<String> {
        let svc = self.repository.lock().await;
            svc.create(&json).await.map_err(err)
    }

    #[napi]
    pub async fn repository_update(&self, id: i64, json: String) -> napi::Result<String> {
        let svc = self.repository.lock().await;
            svc.update(id, &json).await.map_err(err)
    }

    #[napi]
    pub async fn repository_delete(&self, id: i64) -> napi::Result<()> {
        let svc = self.repository.lock().await;
            svc.delete(id).await.map_err(err)
    }

    #[napi]
    pub async fn repository_list_branches(&self, id: i64) -> napi::Result<String> {
        let svc = self.repository.lock().await;
            svc.list_branches(id).await.map_err(err)
    }

    #[napi]
    pub async fn repository_sync_branches(&self, id: i64, json: String) -> napi::Result<String> {
        let svc = self.repository.lock().await;
            svc.sync_branches(id, &json).await.map_err(err)
    }

    #[napi]
    pub async fn repository_register_webhook(&self, id: i64) -> napi::Result<()> {
        let svc = self.repository.lock().await;
            svc.register_webhook(id).await.map_err(err)
    }

    #[napi]
    pub async fn repository_delete_webhook(&self, id: i64) -> napi::Result<()> {
        let svc = self.repository.lock().await;
            svc.delete_webhook(id).await.map_err(err)
    }

    #[napi]
    pub async fn repository_get_webhook_status(&self, id: i64) -> napi::Result<String> {
        let svc = self.repository.lock().await;
            svc.get_webhook_status(id).await.map_err(err)
    }

    #[napi]
    pub async fn repository_get_webhook_secret(&self, id: i64) -> napi::Result<String> {
        let svc = self.repository.lock().await;
            svc.get_webhook_secret(id).await.map_err(err)
    }

    #[napi]
    pub async fn repository_list_merge_requests(&self, id: i64, branch: Option<String>, mr_state: Option<String>) -> napi::Result<String> {
        let svc = self.repository.lock().await;
            svc.list_merge_requests(id, branch, mr_state).await.map_err(err)
    }

    #[napi]
    pub async fn repository_mark_webhook_configured(&self, id: i64) -> napi::Result<()> {
        let svc = self.repository.lock().await;
            svc.mark_webhook_configured(id).await.map_err(err)
    }

    // ===== runner.rs =====
    #[napi]
    pub async fn runner_runners_json(&self) -> napi::Result<String> {
        let svc = self.runner.lock().await;
            Ok(svc.runners_json())
    }

    #[napi]
    pub async fn runner_available_runners_json(&self) -> napi::Result<String> {
        let svc = self.runner.lock().await;
            Ok(svc.available_runners_json())
    }

    #[napi]
    pub async fn runner_current_runner_json(&self) -> napi::Result<String> {
        let svc = self.runner.lock().await;
            Ok(svc.current_runner_json().unwrap_or_default())
    }

    #[napi]
    pub async fn runner_get_runner_json(&self, id: i64) -> napi::Result<String> {
        let svc = self.runner.lock().await;
            Ok(svc.get_runner_json(id).unwrap_or_default())
    }

    #[napi]
    pub async fn runner_set_runners(&self, json: String) -> napi::Result<()> {
        let svc = self.runner.lock().await;
            svc.set_runners(&json);
            Ok(())
    }

    #[napi]
    pub async fn runner_set_available_runners(&self, json: String) -> napi::Result<()> {
        let svc = self.runner.lock().await;
            svc.set_available_runners(&json);
            Ok(())
    }

    #[napi]
    pub async fn runner_set_current_runner(&self, json: String) -> napi::Result<()> {
        let svc = self.runner.lock().await;
            svc.set_current_runner(&json);
            Ok(())
    }

    #[napi]
    pub async fn runner_update_runner_local(&self, id: f64, json: String) -> napi::Result<()> {
        let svc = self.runner.lock().await;
            svc.update_runner_local(id, &json);
            Ok(())
    }

    #[napi]
    pub async fn runner_update_runner_status(&self, id: i64, status: String) -> napi::Result<()> {
        let svc = self.runner.lock().await;
            svc.update_runner_status(id, &status);
            Ok(())
    }

    #[napi]
    pub async fn runner_remove_runner_local(&self, id: i64) -> napi::Result<()> {
        let svc = self.runner.lock().await;
            svc.remove_runner_local(id);
            Ok(())
    }

    // ===== runner_api.rs =====
    #[napi]
    pub async fn runner_fetch_runners(&self, status: Option<String>) -> napi::Result<String> {
        let svc = self.runner.lock().await;
            svc.fetch_runners(status).await.map_err(err)
    }

    #[napi]
    pub async fn runner_fetch_available_runners(&self) -> napi::Result<String> {
        let svc = self.runner.lock().await;
            svc.fetch_available_runners().await.map_err(err)
    }

    #[napi]
    pub async fn runner_fetch_runner(&self, id: i64) -> napi::Result<String> {
        let svc = self.runner.lock().await;
            svc.fetch_runner(id).await.map_err(err)
    }

    #[napi]
    pub async fn runner_update_runner(&self, id: i64, request_json: String) -> napi::Result<String> {
        let svc = self.runner.lock().await;
            svc.update_runner(id, &request_json).await.map_err(err)
    }

    #[napi]
    pub async fn runner_delete_runner(&self, id: i64) -> napi::Result<()> {
        let svc = self.runner.lock().await;
            svc.delete_runner(id).await.map_err(err)
    }

    #[napi]
    pub async fn runner_create_token(&self, request_json: String) -> napi::Result<String> {
        let svc = self.runner.lock().await;
            svc.create_token(&request_json).await.map_err(err)
    }

    #[napi]
    pub async fn runner_fetch_tokens(&self) -> napi::Result<String> {
        let svc = self.runner.lock().await;
            svc.fetch_tokens().await.map_err(err)
    }

    #[napi]
    pub async fn runner_delete_token(&self, id: i64) -> napi::Result<()> {
        let svc = self.runner.lock().await;
            svc.delete_token(id).await.map_err(err)
    }

    #[napi]
    pub async fn runner_list_runner_logs(&self, id: i64) -> napi::Result<String> {
        let svc = self.runner.lock().await;
            svc.list_runner_logs(id).await.map_err(err)
    }

    #[napi]
    pub async fn runner_request_log_upload(&self, id: i64) -> napi::Result<()> {
        let svc = self.runner.lock().await;
            svc.request_log_upload(id).await.map_err(err)
    }

    #[napi]
    pub async fn runner_upgrade_runner(&self, id: i64, request_json: String) -> napi::Result<String> {
        let svc = self.runner.lock().await;
            svc.upgrade_runner(id, &request_json).await.map_err(err)
    }

    #[napi]
    pub async fn runner_list_runner_pods(&self, id: i64, status: Option<String>, limit: Option<u32>, offset: Option<u32>) -> napi::Result<String> {
        let svc = self.runner.lock().await;
            svc.list_runner_pods(id, status, limit, offset).await.map_err(err)
    }

    #[napi]
    pub async fn runner_query_runner_sandboxes(&self, id: i64, request_json: String) -> napi::Result<String> {
        let svc = self.runner.lock().await;
            svc.query_runner_sandboxes(id, &request_json).await.map_err(err)
    }

    #[napi]
    pub async fn runner_get_auth_status(&self, auth_key: String) -> napi::Result<String> {
        let svc = self.runner.lock().await;
            svc.get_auth_status(&auth_key).await.map_err(err)
    }

    #[napi]
    pub async fn runner_authorize_runner(&self, request_json: String) -> napi::Result<String> {
        let svc = self.runner.lock().await;
            svc.authorize_runner(&request_json).await.map_err(err)
    }

    // ===== support_ticket.rs =====
    #[napi]
    pub async fn support_ticket_list(&self, status: Option<String>, page: Option<u32>, page_size: Option<u32>) -> napi::Result<String> {
        let svc = self.support_ticket.lock().await;
            svc.list(status, page, page_size).await.map_err(err)
    }

    #[napi]
    pub async fn support_ticket_get_detail(&self, id: i64) -> napi::Result<String> {
        let svc = self.support_ticket.lock().await;
            svc.get_detail(id).await.map_err(err)
    }

    #[napi]
    pub async fn support_ticket_get_attachment_url(&self, id: i64) -> napi::Result<String> {
        let svc = self.support_ticket.lock().await;
            svc.get_attachment_url(id).await.map_err(err)
    }

    #[napi]
    pub async fn support_ticket_create_ticket(&self, title: String, category: String, content: String, priority: Option<String>, file_data: Vec<Vec<u8>>, file_names: Vec<String>) -> napi::Result<String> {
        let svc = self.support_ticket.lock().await;
            svc.create_ticket(&title, &category, &content, priority, file_data, file_names).await.map_err(err)
    }

    #[napi]
    pub async fn support_ticket_add_message(&self, ticket_id: i64, content: String, file_data: Vec<Vec<u8>>, file_names: Vec<String>) -> napi::Result<String> {
        let svc = self.support_ticket.lock().await;
            svc.add_message(ticket_id, &content, file_data, file_names).await.map_err(err)
    }

    // ===== ticket.rs =====
    #[napi]
    pub async fn ticket_tickets_json(&self) -> napi::Result<String> {
        let svc = self.ticket.lock().await;
            Ok(svc.tickets_json())
    }

    #[napi]
    pub async fn ticket_get_ticket_by_slug_json(&self, slug: String) -> napi::Result<String> {
        let svc = self.ticket.lock().await;
            Ok(svc.get_ticket_by_slug_json(&slug).unwrap_or_default())
    }

    #[napi]
    pub async fn ticket_current_ticket_json(&self) -> napi::Result<String> {
        let svc = self.ticket.lock().await;
            Ok(svc.current_ticket_json().unwrap_or_default())
    }

    #[napi]
    pub async fn ticket_board_columns_json(&self) -> napi::Result<String> {
        let svc = self.ticket.lock().await;
            Ok(svc.board_columns_json())
    }

    #[napi]
    pub async fn ticket_labels_json(&self) -> napi::Result<String> {
        let svc = self.ticket.lock().await;
            Ok(svc.labels_json())
    }

    #[napi]
    pub async fn ticket_filter_tickets_json(&self, search: String, statuses_json: String, priorities_json: String, repository_ids_json: String) -> napi::Result<String> {
        let svc = self.ticket.lock().await;
            Ok(svc.filter_tickets_json(&search, &statuses_json, &priorities_json, &repository_ids_json))
    }

    #[napi]
    pub async fn ticket_set_tickets(&self, json: String) -> napi::Result<()> {
        let svc = self.ticket.lock().await;
            svc.set_tickets(&json);
            Ok(())
    }

    #[napi]
    pub async fn ticket_add_ticket(&self, json: String) -> napi::Result<()> {
        let svc = self.ticket.lock().await;
            svc.add_ticket(&json);
            Ok(())
    }

    #[napi]
    pub async fn ticket_update_ticket_local(&self, slug: String, json: String) -> napi::Result<()> {
        let svc = self.ticket.lock().await;
            svc.update_ticket_local(&slug, &json);
            Ok(())
    }

    #[napi]
    pub async fn ticket_update_ticket_status_local(&self, slug: String, status: String) -> napi::Result<()> {
        let svc = self.ticket.lock().await;
            svc.update_ticket_status_local(&slug, &status);
            Ok(())
    }

    #[napi]
    pub async fn ticket_remove_ticket(&self, slug: String) -> napi::Result<()> {
        let svc = self.ticket.lock().await;
            svc.remove_ticket(&slug);
            Ok(())
    }

    #[napi]
    pub async fn ticket_set_current_ticket(&self, json: String) -> napi::Result<()> {
        let svc = self.ticket.lock().await;
            svc.set_current_ticket(&json);
            Ok(())
    }

    #[napi]
    pub async fn ticket_set_board_columns(&self, json: String) -> napi::Result<()> {
        let svc = self.ticket.lock().await;
            svc.set_board_columns(&json);
            Ok(())
    }

    #[napi]
    pub async fn ticket_append_column_tickets(&self, status: String, json: String) -> napi::Result<()> {
        let svc = self.ticket.lock().await;
            svc.append_column_tickets(&status, &json);
            Ok(())
    }

    #[napi]
    pub async fn ticket_set_labels(&self, json: String) -> napi::Result<()> {
        let svc = self.ticket.lock().await;
            svc.set_labels(&json);
            Ok(())
    }

    #[napi]
    pub async fn ticket_add_label(&self, json: String) -> napi::Result<()> {
        let svc = self.ticket.lock().await;
            svc.add_label(&json);
            Ok(())
    }

    #[napi]
    pub async fn ticket_remove_label(&self, id: f64) -> napi::Result<()> {
        let svc = self.ticket.lock().await;
            svc.remove_label(id);
            Ok(())
    }

    // ===== ticket_api.rs =====
    #[napi]
    pub async fn ticket_fetch_tickets(&self, status: Option<String>, limit: Option<u32>, offset: Option<u32>) -> napi::Result<String> {
        let svc = self.ticket.lock().await;
            svc.fetch_tickets(status, limit, offset).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_fetch_board(&self, repository_id: Option<i64>) -> napi::Result<String> {
        let svc = self.ticket.lock().await;
            svc.fetch_board(repository_id).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_load_more_column(&self, status: String, offset: u32, limit: u32) -> napi::Result<String> {
        let svc = self.ticket.lock().await;
            svc.load_more_column(&status, offset, limit).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_fetch_ticket(&self, slug: String) -> napi::Result<String> {
        let svc = self.ticket.lock().await;
            svc.fetch_ticket(&slug).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_create_ticket(&self, request_json: String) -> napi::Result<String> {
        let svc = self.ticket.lock().await;
            svc.create_ticket(&request_json).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_update_ticket(&self, slug: String, request_json: String) -> napi::Result<String> {
        let svc = self.ticket.lock().await;
            svc.update_ticket(&slug, &request_json).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_delete_ticket(&self, slug: String) -> napi::Result<()> {
        let svc = self.ticket.lock().await;
            svc.delete_ticket(&slug).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_update_ticket_status(&self, slug: String, status: String) -> napi::Result<String> {
        let svc = self.ticket.lock().await;
            svc.update_ticket_status(&slug, &status).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_fetch_labels(&self, repository_id: Option<i64>) -> napi::Result<String> {
        let svc = self.ticket.lock().await;
            svc.fetch_labels(repository_id).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_create_label(&self, name: String, color: String, repository_id: Option<i64>) -> napi::Result<String> {
        let svc = self.ticket.lock().await;
            svc.create_label(&name, &color, repository_id).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_delete_label(&self, id: f64) -> napi::Result<()> {
        let svc = self.ticket.lock().await;
            svc.delete_label(id).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_get_ticket_pods(&self, slug: String, active_only: Option<bool>) -> napi::Result<String> {
        let svc = self.ticket.lock().await;
            svc.get_ticket_pods(&slug, active_only).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_get_sub_tickets(&self, slug: String) -> napi::Result<String> {
        let svc = self.ticket.lock().await;
            svc.get_sub_tickets(&slug).await.map_err(err)
    }

    // ===== ticket_relations.rs =====
    #[napi]
    pub async fn ticket_relations_list_relations(&self, slug: String) -> napi::Result<String> {
        let svc = self.ticket_relations.lock().await;
            svc.list_relations(&slug).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_relations_create_relation(&self, slug: String, json: String) -> napi::Result<String> {
        let svc = self.ticket_relations.lock().await;
            svc.create_relation(&slug, &json).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_relations_delete_relation(&self, slug: String, relation_id: i64) -> napi::Result<()> {
        let svc = self.ticket_relations.lock().await;
            svc.delete_relation(&slug, relation_id).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_relations_list_commits(&self, slug: String) -> napi::Result<String> {
        let svc = self.ticket_relations.lock().await;
            svc.list_commits(&slug).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_relations_link_commit(&self, slug: String, json: String) -> napi::Result<String> {
        let svc = self.ticket_relations.lock().await;
            svc.link_commit(&slug, &json).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_relations_unlink_commit(&self, slug: String, commit_id: i64) -> napi::Result<()> {
        let svc = self.ticket_relations.lock().await;
            svc.unlink_commit(&slug, commit_id).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_relations_list_merge_requests(&self, slug: String) -> napi::Result<String> {
        let svc = self.ticket_relations.lock().await;
            svc.list_merge_requests(&slug).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_relations_list_comments(&self, slug: String, limit: Option<u32>, offset: Option<u32>) -> napi::Result<String> {
        let svc = self.ticket_relations.lock().await;
            svc.list_comments(&slug, limit, offset).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_relations_create_comment(&self, slug: String, json: String) -> napi::Result<String> {
        let svc = self.ticket_relations.lock().await;
            svc.create_comment(&slug, &json).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_relations_update_comment(&self, slug: String, comment_id: i64, json: String) -> napi::Result<String> {
        let svc = self.ticket_relations.lock().await;
            svc.update_comment(&slug, comment_id, &json).await.map_err(err)
    }

    #[napi]
    pub async fn ticket_relations_delete_comment(&self, slug: String, comment_id: i64) -> napi::Result<()> {
        let svc = self.ticket_relations.lock().await;
            svc.delete_comment(&slug, comment_id).await.map_err(err)
    }

    // ===== user.rs =====
    #[napi]
    pub async fn user_get_me(&self) -> napi::Result<String> {
        let svc = self.user.lock().await;
            svc.get_me().await.map_err(err)
    }

    #[napi]
    pub async fn user_get_organizations(&self) -> napi::Result<String> {
        let svc = self.user.lock().await;
            svc.get_organizations().await.map_err(err)
    }

    // ===== user_credential.rs =====
    #[napi]
    pub async fn user_credential_list_git_credentials(&self) -> napi::Result<String> {
        let svc = self.user_credential.lock().await;
            svc.list_git_credentials().await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_create_git_credential(&self, json: String) -> napi::Result<String> {
        let svc = self.user_credential.lock().await;
            svc.create_git_credential(&json).await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_get_git_credential(&self, id: i64) -> napi::Result<String> {
        let svc = self.user_credential.lock().await;
            svc.get_git_credential(id).await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_update_git_credential(&self, id: i64, json: String) -> napi::Result<String> {
        let svc = self.user_credential.lock().await;
            svc.update_git_credential(id, &json).await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_delete_git_credential(&self, id: i64) -> napi::Result<()> {
        let svc = self.user_credential.lock().await;
            svc.delete_git_credential(id).await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_get_default_git_credential(&self) -> napi::Result<String> {
        let svc = self.user_credential.lock().await;
            svc.get_default_git_credential().await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_set_default_git_credential(&self, json: String) -> napi::Result<()> {
        let svc = self.user_credential.lock().await;
            svc.set_default_git_credential(&json).await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_clear_default_git_credential(&self) -> napi::Result<()> {
        let svc = self.user_credential.lock().await;
            svc.clear_default_git_credential().await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_list_agent_credentials(&self) -> napi::Result<String> {
        let svc = self.user_credential.lock().await;
            svc.list_agent_credentials().await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_list_agent_credentials_for_agent(&self, agent_slug: String) -> napi::Result<String> {
        let svc = self.user_credential.lock().await;
            svc.list_agent_credentials_for_agent(&agent_slug).await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_create_agent_credential(&self, agent_slug: String, json: String) -> napi::Result<String> {
        let svc = self.user_credential.lock().await;
            svc.create_agent_credential(&agent_slug, &json).await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_get_agent_credential(&self, id: i64) -> napi::Result<String> {
        let svc = self.user_credential.lock().await;
            svc.get_agent_credential(id).await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_update_agent_credential(&self, id: i64, json: String) -> napi::Result<String> {
        let svc = self.user_credential.lock().await;
            svc.update_agent_credential(id, &json).await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_delete_agent_credential(&self, id: i64) -> napi::Result<()> {
        let svc = self.user_credential.lock().await;
            svc.delete_agent_credential(id).await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_set_default_agent_credential(&self, id: i64) -> napi::Result<()> {
        let svc = self.user_credential.lock().await;
            svc.set_default_agent_credential(id).await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_list_repo_providers(&self) -> napi::Result<String> {
        let svc = self.user_credential.lock().await;
            svc.list_repo_providers().await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_create_repo_provider(&self, json: String) -> napi::Result<String> {
        let svc = self.user_credential.lock().await;
            svc.create_repo_provider(&json).await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_get_repo_provider(&self, id: i64) -> napi::Result<String> {
        let svc = self.user_credential.lock().await;
            svc.get_repo_provider(id).await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_update_repo_provider(&self, id: i64, json: String) -> napi::Result<String> {
        let svc = self.user_credential.lock().await;
            svc.update_repo_provider(id, &json).await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_delete_repo_provider(&self, id: i64) -> napi::Result<()> {
        let svc = self.user_credential.lock().await;
            svc.delete_repo_provider(id).await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_set_default_repo_provider(&self, id: i64) -> napi::Result<()> {
        let svc = self.user_credential.lock().await;
            svc.set_default_repo_provider(id).await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_test_repo_provider(&self, id: i64) -> napi::Result<()> {
        let svc = self.user_credential.lock().await;
            svc.test_repo_provider(id).await.map_err(err)
    }

    #[napi]
    pub async fn user_credential_list_provider_repositories(&self, id: i64, page: Option<u32>, per_page: Option<u32>, search: Option<String>) -> napi::Result<String> {
        let svc = self.user_credential.lock().await;
            svc.list_provider_repositories(id, page, per_page, search).await.map_err(err)
    }

    // ===== token_usage (manually added — missing from gen script) =====
    #[napi]
    pub async fn token_usage_get_dashboard(
        &self,
        start_time: Option<String>,
        end_time: Option<String>,
        agent_slug: Option<String>,
        user_id: Option<i64>,
        model: Option<String>,
        granularity: Option<String>,
    ) -> napi::Result<String> {
        let svc = self.token_usage.lock().await;
        svc.get_dashboard(start_time, end_time, agent_slug, user_id, model, granularity)
            .await
            .map_err(err)
    }

}
