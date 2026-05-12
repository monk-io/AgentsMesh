use agentsmesh_types::proto_user_credential_v1 as uc_proto;
use prost::Message;

use super::user_credential::UserCredentialService;

// Connect-RPC bridge (binary wire) for the three user_credential.v1 services.
// Each method accepts a prost-encoded request body and returns a prost-encoded
// response body — wasm-bridge surface is `Result<Vec<u8>, String>`
// (conventions §2.5).
//
// Three services share this impl block because they share the proto package
// and the same WasmUserCredentialService bridge wraps them.

impl UserCredentialService {
    // -------- UserGitCredentialService (8 RPCs) --------

    pub async fn list_git_credentials_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let _ = uc_proto::ListGitCredentialsRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_git_credentials request: {e}"))?;
        let resp = self.client().list_git_credentials_connect().await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_git_credential_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = uc_proto::GetGitCredentialRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_git_credential request: {e}"))?;
        let resp = self.client().get_git_credential_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn create_git_credential_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = uc_proto::CreateGitCredentialRequest::decode(request_bytes)
            .map_err(|e| format!("decode create_git_credential request: {e}"))?;
        let resp = self.client().create_git_credential_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn update_git_credential_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = uc_proto::UpdateGitCredentialRequest::decode(request_bytes)
            .map_err(|e| format!("decode update_git_credential request: {e}"))?;
        let resp = self.client().update_git_credential_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn delete_git_credential_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = uc_proto::DeleteGitCredentialRequest::decode(request_bytes)
            .map_err(|e| format!("decode delete_git_credential request: {e}"))?;
        let resp = self.client().delete_git_credential_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_default_git_credential_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let _ = uc_proto::GetDefaultGitCredentialRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_default_git_credential request: {e}"))?;
        let resp = self.client().get_default_git_credential_connect().await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn set_default_git_credential_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = uc_proto::SetDefaultGitCredentialRequest::decode(request_bytes)
            .map_err(|e| format!("decode set_default_git_credential request: {e}"))?;
        let resp = self.client().set_default_git_credential_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn clear_default_git_credential_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let _ = uc_proto::ClearDefaultGitCredentialRequest::decode(request_bytes)
            .map_err(|e| format!("decode clear_default_git_credential request: {e}"))?;
        let resp = self.client().clear_default_git_credential_connect().await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }
}
