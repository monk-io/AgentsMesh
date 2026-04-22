use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Agent {
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub is_builtin: Option<bool>,
    #[serde(default)]
    pub icon_url: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default, deserialize_with = "deserialize_modes")]
    pub supported_modes: Option<Vec<String>>,
}

fn deserialize_modes<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where D: serde::Deserializer<'de> {
    use serde::de;
    struct ModesVisitor;
    impl<'de> de::Visitor<'de> for ModesVisitor {
        type Value = Option<Vec<String>>;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a string or array of strings")
        }
        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> { Ok(None) }
        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> { Ok(None) }
        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            Ok(Some(v.split(',').map(|s| s.trim().to_string()).collect()))
        }
        fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
            self.visit_str(&v)
        }
        fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let mut v = Vec::new();
            while let Some(s) = seq.next_element::<String>()? { v.push(s); }
            Ok(Some(v))
        }
    }
    deserializer.deserialize_any(ModesVisitor)
}

pub type AgentConfigSchema = serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAgentConfig {
    pub agent_slug: String,
    pub config_values: Option<serde_json::Value>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetUserAgentConfigRequest {
    pub config_values: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentListResponse {
    #[serde(default)]
    pub agents: Vec<Agent>,
    #[serde(default)]
    pub builtin_agents: Vec<Agent>,
    #[serde(default)]
    pub custom_agents: Vec<Agent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAgentConfigListResponse {
    pub configs: Vec<UserAgentConfig>,
}
