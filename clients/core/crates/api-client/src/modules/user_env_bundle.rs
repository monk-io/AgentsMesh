use crate::ApiClient;
use crate::error::ApiError;
use agentsmesh_types::*;

/// Lightweight URL-safe escape for query-string values we control on the
/// frontend (bundle kind / agent slug). Both inputs are validated upstream
/// (kind is a code-defined string, agent_slug matches `[a-z0-9-]+`), so we
/// only need to handle the obvious cases plus `%`/`+`/`&` defensively. We
/// avoid a full urlencoding dep here to keep the crate graph minimal.
fn escape_query(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => out.push(ch),
            other => {
                for b in other.to_string().as_bytes() {
                    out.push_str(&format!("%{:02X}", b));
                }
            }
        }
    }
    out
}

impl ApiClient {
    pub async fn list_user_env_bundles(
        &self,
        kind: Option<&str>,
        agent_slug: Option<&str>,
    ) -> Result<EnvBundleListResponse, ApiError> {
        let mut query = Vec::new();
        if let Some(k) = kind {
            query.push(format!("kind={}", escape_query(k)));
        }
        if let Some(s) = agent_slug {
            query.push(format!("agent_slug={}", escape_query(s)));
        }
        let url = if query.is_empty() {
            "/api/v1/users/env-bundles".to_string()
        } else {
            format!("/api/v1/users/env-bundles?{}", query.join("&"))
        };
        self.get(&url).await
    }

    pub async fn get_user_env_bundle(&self, id: i64) -> Result<EnvBundle, ApiError> {
        self.get_resource(&format!("/api/v1/users/env-bundles/{id}"), "bundle")
            .await
    }

    pub async fn create_user_env_bundle(
        &self,
        data: &CreateEnvBundleRequest,
    ) -> Result<EnvBundle, ApiError> {
        self.post_resource("/api/v1/users/env-bundles", data, "bundle")
            .await
    }

    pub async fn update_user_env_bundle(
        &self,
        id: i64,
        data: &UpdateEnvBundleRequest,
    ) -> Result<EnvBundle, ApiError> {
        self.put_resource(&format!("/api/v1/users/env-bundles/{id}"), data, "bundle")
            .await
    }

    pub async fn delete_user_env_bundle(&self, id: i64) -> Result<EmptyResponse, ApiError> {
        self.delete(&format!("/api/v1/users/env-bundles/{id}")).await
    }

    pub async fn set_primary_env_bundle(&self, id: i64) -> Result<EnvBundle, ApiError> {
        self.post_resource(
            &format!("/api/v1/users/env-bundles/{id}/set-primary"),
            &serde_json::json!({}),
            "bundle",
        )
        .await
    }
}
