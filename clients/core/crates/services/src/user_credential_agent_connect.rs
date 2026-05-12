use agentsmesh_types::proto_user_credential_v1 as uc_proto;
use prost::Message;

use super::user_credential::UserCredentialService;

impl UserCredentialService {
    // -------- UserAgentCredentialService (7 RPCs) --------

    pub async fn list_agent_credentials_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let _ = uc_proto::ListAgentCredentialProfilesRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_agent_credentials request: {e}"))?;
        let resp = self.client().list_agent_credentials_connect().await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn list_agent_credentials_for_agent_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = uc_proto::ListAgentCredentialProfilesForAgentRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_agent_credentials_for_agent request: {e}"))?;
        let resp = self.client().list_agent_credentials_for_agent_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_agent_credential_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = uc_proto::GetAgentCredentialProfileRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_agent_credential request: {e}"))?;
        let resp = self.client().get_agent_credential_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn create_agent_credential_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = uc_proto::CreateAgentCredentialProfileRequest::decode(request_bytes)
            .map_err(|e| format!("decode create_agent_credential request: {e}"))?;
        let resp = self.client().create_agent_credential_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn update_agent_credential_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = uc_proto::UpdateAgentCredentialProfileRequest::decode(request_bytes)
            .map_err(|e| format!("decode update_agent_credential request: {e}"))?;
        let resp = self.client().update_agent_credential_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn delete_agent_credential_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = uc_proto::DeleteAgentCredentialProfileRequest::decode(request_bytes)
            .map_err(|e| format!("decode delete_agent_credential request: {e}"))?;
        let resp = self.client().delete_agent_credential_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn set_default_agent_credential_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = uc_proto::SetDefaultAgentCredentialProfileRequest::decode(request_bytes)
            .map_err(|e| format!("decode set_default_agent_credential request: {e}"))?;
        let resp = self.client().set_default_agent_credential_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }
}
