// Hand-maintained `prost::Message` mirrors of `proto/agent/v1/agent.proto`.
// Tag numbers match the .proto byte-for-byte; `tools/validate_prost_tags`
// runs at build time to catch drift (watch list §8). NO
// `Serialize`/`Deserialize` derives on these structs — binary wire only
// (conventions §2.5, §3).
//
// Coexists with the legacy serde `Agent` / `UserAgentConfig` /
// `AgentListResponse` / `UserAgentConfigListResponse` in `agent.rs` for the
// dual-track migration window. Re-exported under `proto_agent_v1` in lib.rs.

#[derive(Clone, PartialEq, prost::Message)]
pub struct Agent {
    #[prost(string, tag = "1")]
    pub slug: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(string, optional, tag = "3")]
    pub description: Option<String>,
    #[prost(string, tag = "4")]
    pub launch_command: String,
    #[prost(string, optional, tag = "5")]
    pub executable: Option<String>,
    #[prost(string, optional, tag = "6")]
    pub default_args: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub agentfile_source: Option<String>,
    #[prost(bool, tag = "8")]
    pub is_builtin: bool,
    #[prost(bool, tag = "9")]
    pub is_active: bool,
    #[prost(string, tag = "10")]
    pub supported_modes: String,
    #[prost(string, tag = "11")]
    pub created_at: String,
    #[prost(string, tag = "12")]
    pub updated_at: String,
    #[prost(int64, optional, tag = "13")]
    pub organization_id: Option<i64>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct AgentListResponse {
    #[prost(message, repeated, tag = "1")]
    pub builtin_agents: Vec<Agent>,
    #[prost(message, repeated, tag = "2")]
    pub custom_agents: Vec<Agent>,
    #[prost(message, repeated, tag = "3")]
    pub agents: Vec<Agent>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListAgentsRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetAgentRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub agent_slug: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CreateCustomAgentRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub slug: String,
    #[prost(string, tag = "3")]
    pub name: String,
    #[prost(string, optional, tag = "4")]
    pub description: Option<String>,
    #[prost(string, optional, tag = "5")]
    pub agentfile_source: Option<String>,
    #[prost(string, optional, tag = "6")]
    pub launch_command: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub default_args: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UpdateCustomAgentRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub agent_slug: String,
    #[prost(string, tag = "3")]
    pub updates_json: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteCustomAgentRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub agent_slug: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteCustomAgentResponse {
    #[prost(string, tag = "1")]
    pub message: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ConfigSchema {
    #[prost(message, repeated, tag = "1")]
    pub fields: Vec<ConfigField>,
    #[prost(message, repeated, tag = "2")]
    pub credential_fields: Vec<CredentialField>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ConfigField {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub r#type: String,
    #[prost(string, tag = "3")]
    pub default_json: String,
    #[prost(message, repeated, tag = "4")]
    pub options: Vec<FieldOption>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct FieldOption {
    #[prost(string, tag = "1")]
    pub value: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct CredentialField {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub r#type: String,
    #[prost(bool, tag = "3")]
    pub optional: bool,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetAgentConfigSchemaRequest {
    #[prost(string, tag = "1")]
    pub org_slug: String,
    #[prost(string, tag = "2")]
    pub agent_slug: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UserAgentConfig {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub user_id: i64,
    #[prost(string, tag = "3")]
    pub agent_slug: String,
    #[prost(string, optional, tag = "4")]
    pub agent_name: Option<String>,
    #[prost(string, tag = "5")]
    pub config_values_json: String,
    #[prost(string, tag = "6")]
    pub created_at: String,
    #[prost(string, tag = "7")]
    pub updated_at: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct UserAgentConfigListResponse {
    #[prost(message, repeated, tag = "1")]
    pub configs: Vec<UserAgentConfig>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListUserAgentConfigsRequest {}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetUserAgentConfigRequest {
    #[prost(string, tag = "1")]
    pub agent_slug: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SetUserAgentConfigRequest {
    #[prost(string, tag = "1")]
    pub agent_slug: String,
    #[prost(string, tag = "2")]
    pub config_values_json: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteUserAgentConfigRequest {
    #[prost(string, tag = "1")]
    pub agent_slug: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct DeleteUserAgentConfigResponse {
    #[prost(string, tag = "1")]
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    fn sample_agent() -> Agent {
        Agent {
            slug: "claude-code".into(),
            name: "Claude Code".into(),
            description: Some("AI coding agent".into()),
            launch_command: "claude".into(),
            executable: Some("claude".into()),
            default_args: Some("--no-color".into()),
            agentfile_source: Some("AGENT claude\nMODE pty\n".into()),
            is_builtin: true,
            is_active: true,
            supported_modes: "pty,acp".into(),
            created_at: "2026-05-08T00:00:00Z".into(),
            updated_at: "2026-05-09T00:00:00Z".into(),
            organization_id: None,
        }
    }

    fn sample_custom_agent() -> Agent {
        Agent {
            slug: "my-agent".into(),
            name: "My Custom".into(),
            description: None,
            launch_command: "./run".into(),
            executable: None,
            default_args: None,
            agentfile_source: None,
            is_builtin: false,
            is_active: true,
            supported_modes: "pty".into(),
            created_at: "2026-05-08T00:00:00Z".into(),
            updated_at: "2026-05-08T00:00:00Z".into(),
            organization_id: Some(42),
        }
    }

    #[test]
    fn agent_round_trip_preserves_every_field() {
        let original = sample_agent();
        let bytes = original.encode_to_vec();
        let decoded = Agent::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.supported_modes, "pty,acp");
        assert!(decoded.is_builtin);
        assert_eq!(decoded.organization_id, None);
    }

    #[test]
    fn custom_agent_round_trip_preserves_organization_id() {
        let original = sample_custom_agent();
        let bytes = original.encode_to_vec();
        let decoded = Agent::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.organization_id, Some(42));
        assert!(!decoded.is_builtin);
    }

    // AgentListResponse §9 multi-field exception — both slices must survive
    // the wire so the UI can render builtin vs custom separately.
    #[test]
    fn agent_list_response_preserves_split() {
        let original = AgentListResponse {
            builtin_agents: vec![sample_agent()],
            custom_agents: vec![sample_custom_agent()],
            agents: vec![sample_agent(), sample_custom_agent()],
        };
        let bytes = original.encode_to_vec();
        let decoded = AgentListResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.builtin_agents.len(), 1);
        assert_eq!(decoded.custom_agents.len(), 1);
        assert_eq!(decoded.agents.len(), 2);
    }

    #[test]
    fn config_schema_round_trip() {
        let original = ConfigSchema {
            fields: vec![ConfigField {
                name: "model".into(),
                r#type: "select".into(),
                default_json: "\"sonnet\"".into(),
                options: vec![
                    FieldOption { value: "sonnet".into() },
                    FieldOption { value: "opus".into() },
                ],
            }],
            credential_fields: vec![CredentialField {
                name: "ANTHROPIC_API_KEY".into(),
                r#type: "secret".into(),
                optional: false,
            }],
        };
        let bytes = original.encode_to_vec();
        let decoded = ConfigSchema::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.fields[0].options.len(), 2);
    }

    #[test]
    fn user_agent_config_round_trip_preserves_json_blob() {
        let original = UserAgentConfig {
            id: 7,
            user_id: 99,
            agent_slug: "claude-code".into(),
            agent_name: Some("Claude Code".into()),
            config_values_json: r#"{"model":"sonnet","verbose":true}"#.into(),
            created_at: "2026-05-08T00:00:00Z".into(),
            updated_at: "2026-05-09T00:00:00Z".into(),
        };
        let bytes = original.encode_to_vec();
        let decoded = UserAgentConfig::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.config_values_json, r#"{"model":"sonnet","verbose":true}"#);
    }

    // Sub-resource envelope (§9 exception #2) — `configs` field name is
    // intentionally kept (not `items`) to preserve the dual-track REST shape.
    #[test]
    fn user_agent_config_list_response_uses_configs_field() {
        let original = UserAgentConfigListResponse {
            configs: vec![UserAgentConfig {
                id: 1,
                user_id: 1,
                agent_slug: "claude".into(),
                agent_name: None,
                config_values_json: "{}".into(),
                created_at: "2026-05-08T00:00:00Z".into(),
                updated_at: "2026-05-08T00:00:00Z".into(),
            }],
        };
        let bytes = original.encode_to_vec();
        let decoded = UserAgentConfigListResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.configs.len(), 1);
    }

    #[test]
    fn create_custom_agent_request_round_trip() {
        let original = CreateCustomAgentRequest {
            org_slug: "acme".into(),
            slug: "my-agent".into(),
            name: "My Agent".into(),
            description: Some("Custom AI".into()),
            agentfile_source: Some("AGENT myagent\n".into()),
            launch_command: None,
            default_args: None,
        };
        let bytes = original.encode_to_vec();
        let decoded = CreateCustomAgentRequest::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn empty_messages_round_trip() {
        let req = ListUserAgentConfigsRequest {};
        let bytes = req.encode_to_vec();
        assert!(bytes.is_empty(), "empty message encodes to zero bytes");
        assert_eq!(req, ListUserAgentConfigsRequest::decode(&*bytes).unwrap());
    }
}
