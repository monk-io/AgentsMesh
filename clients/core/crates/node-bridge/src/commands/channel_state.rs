use napi_derive::napi;
use crate::AppState;

// NAPI bridge for the channel cache. Each command forwards to the Rust
// `ChannelService` running on the desktop main process. Mutation commands
// take proto-encoded `Buffer`s (matching the wasm web bridge); read-side
// commands are still JSON for the renderer-side selector layer.
//
// Legacy JSON commands (channel_set_messages / channel_on_new_message /
// channel_update_message_local / channel_set_unread_counts ...) were
// removed when the renderer migrated to proto. If you need to call them
// from the desktop main process, encode the proto request via
// @bufbuild/protobuf and use the `*_proto` variant below.
#[napi]
impl AppState {
    #[napi]
    pub async fn channel_set_current_user_id(&self, user_id: Option<i64>) -> napi::Result<()> {
        let _ = user_id;
        // current_user_id is part of the legacy cache path; no remaining
        // desktop caller. Kept as a no-op so any stale renderer call
        // doesn't crash the host.
        Ok(())
    }

    #[napi]
    pub async fn channel_replace_cached_channel_messages(
        &self, req_bytes: Vec<u8>,
    ) -> napi::Result<()> {
        let svc = self.channel.lock().await;
        svc.replace_cached_channel_messages(&req_bytes)
            .map_err(|e| napi::Error::from_reason(e))
    }

    #[napi]
    pub async fn channel_prepend_cached_channel_messages(
        &self, req_bytes: Vec<u8>,
    ) -> napi::Result<()> {
        let svc = self.channel.lock().await;
        svc.prepend_cached_channel_messages(&req_bytes)
            .map_err(|e| napi::Error::from_reason(e))
    }

    #[napi]
    pub async fn channel_insert_channel_message(
        &self, req_bytes: Vec<u8>,
    ) -> napi::Result<()> {
        let svc = self.channel.lock().await;
        svc.insert_channel_message(&req_bytes)
            .map_err(|e| napi::Error::from_reason(e))
    }

    #[napi]
    pub async fn channel_apply_incoming_channel_message(
        &self, req_bytes: Vec<u8>,
    ) -> napi::Result<bool> {
        let svc = self.channel.lock().await;
        svc.apply_incoming_channel_message(&req_bytes)
            .map_err(|e| napi::Error::from_reason(e))
    }

    #[napi]
    pub async fn channel_apply_channel_message_edited_event(
        &self, req_bytes: Vec<u8>,
    ) -> napi::Result<()> {
        let svc = self.channel.lock().await;
        svc.apply_channel_message_edited_event(&req_bytes)
            .map_err(|e| napi::Error::from_reason(e))
    }

    #[napi]
    pub async fn channel_remove_message(
        &self, channel_id: i64, message_id: i64,
    ) -> napi::Result<()> {
        let svc = self.channel.lock().await;
        svc.remove_message(channel_id, message_id);
        Ok(())
    }

    #[napi]
    pub async fn channel_replace_channel_unread_counts(
        &self, req_bytes: Vec<u8>,
    ) -> napi::Result<()> {
        let svc = self.channel.lock().await;
        svc.replace_channel_unread_counts(&req_bytes)
            .map_err(|e| napi::Error::from_reason(e))
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
}
