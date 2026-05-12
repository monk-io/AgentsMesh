use agentsmesh_types::proto_user_credential_v1 as uc_proto;
use prost::Message;

use super::user_credential::UserCredentialService;

impl UserCredentialService {
    // -------- UserRepositoryProviderService (8 RPCs) --------

    pub async fn list_repository_providers_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let _ = uc_proto::ListRepositoryProvidersRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_repository_providers request: {e}"))?;
        let resp = self.client().list_repository_providers_connect().await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn get_repository_provider_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = uc_proto::GetRepositoryProviderRequest::decode(request_bytes)
            .map_err(|e| format!("decode get_repository_provider request: {e}"))?;
        let resp = self.client().get_repository_provider_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn create_repository_provider_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = uc_proto::CreateRepositoryProviderRequest::decode(request_bytes)
            .map_err(|e| format!("decode create_repository_provider request: {e}"))?;
        let resp = self.client().create_repository_provider_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn update_repository_provider_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = uc_proto::UpdateRepositoryProviderRequest::decode(request_bytes)
            .map_err(|e| format!("decode update_repository_provider request: {e}"))?;
        let resp = self.client().update_repository_provider_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn delete_repository_provider_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = uc_proto::DeleteRepositoryProviderRequest::decode(request_bytes)
            .map_err(|e| format!("decode delete_repository_provider request: {e}"))?;
        let resp = self.client().delete_repository_provider_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn set_default_repository_provider_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = uc_proto::SetDefaultRepositoryProviderRequest::decode(request_bytes)
            .map_err(|e| format!("decode set_default_repository_provider request: {e}"))?;
        let resp = self.client().set_default_repository_provider_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn test_repository_provider_connection_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = uc_proto::TestRepositoryProviderConnectionRequest::decode(request_bytes)
            .map_err(|e| format!("decode test_repository_provider_connection request: {e}"))?;
        let resp = self.client().test_repository_provider_connection_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }

    pub async fn list_provider_repositories_connect(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let req = uc_proto::ListProviderRepositoriesRequest::decode(request_bytes)
            .map_err(|e| format!("decode list_provider_repositories request: {e}"))?;
        let resp = self.client().list_provider_repositories_connect(&req).await.map_err(crate::wire)?;
        Ok(resp.encode_to_vec())
    }
}
