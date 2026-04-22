use std::sync::Arc;

use agentsmesh_types::Runner;

use crate::backend::StorageBackend;
use crate::error::Result;

pub struct RunnerRepo {
    backend: Arc<dyn StorageBackend>,
}

impl RunnerRepo {
    pub fn new(backend: Arc<dyn StorageBackend>) -> Self {
        Self { backend }
    }

    pub fn get(&self, id: i64) -> Result<Option<Runner>> {
        match self.backend.get_raw("runners", &id.to_string())? {
            Some(data) => Ok(Some(serde_json::from_slice(&data)?)),
            None => Ok(None),
        }
    }

    pub fn save(&self, runner: &Runner) -> Result<()> {
        let data = serde_json::to_vec(runner)?;
        let status_str = runner.status.to_string();
        let fields: &[(&str, &str)] = &[("status", &status_str)];
        self.backend
            .put_raw("runners", &runner.id.to_string(), fields, &data)
    }

    pub fn delete(&self, id: i64) -> Result<()> {
        self.backend.delete_raw("runners", &id.to_string())
    }

    pub fn get_by_status(&self, status: &str) -> Result<Vec<Runner>> {
        super::deserialize_rows(self.backend.query_raw("runners", "status", status)?)
    }

    pub fn list_all(&self) -> Result<Vec<Runner>> {
        super::deserialize_rows(self.backend.list_raw("runners")?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::InMemoryBackend;

    fn make_repo() -> RunnerRepo {
        RunnerRepo::new(Arc::new(InMemoryBackend::new()))
    }

    fn make_runner(id: i64, status: agentsmesh_types::RunnerStatus) -> Runner {
        Runner {
            id,
            name: format!("runner-{id}"),
            status,
            version: None,
            max_concurrent_pods: 4,
            active_pod_count: 0,
            is_enabled: true,
            host_info: None,
            created_at: None,
            updated_at: None,
        }
    }

    #[test]
    fn crud_roundtrip() {
        let repo = make_repo();
        repo.save(&make_runner(1, agentsmesh_types::RunnerStatus::Online)).unwrap();
        let loaded = repo.get(1).unwrap().unwrap();
        assert_eq!(loaded.name, "runner-1");
        repo.delete(1).unwrap();
        assert!(repo.get(1).unwrap().is_none());
    }

    #[test]
    fn filter_by_status() {
        let repo = make_repo();
        repo.save(&make_runner(1, agentsmesh_types::RunnerStatus::Online)).unwrap();
        repo.save(&make_runner(2, agentsmesh_types::RunnerStatus::Offline)).unwrap();
        repo.save(&make_runner(3, agentsmesh_types::RunnerStatus::Online)).unwrap();
        assert_eq!(repo.get_by_status("online").unwrap().len(), 2);
    }

    #[test]
    fn list_all() {
        let repo = make_repo();
        repo.save(&make_runner(1, agentsmesh_types::RunnerStatus::Online)).unwrap();
        repo.save(&make_runner(2, agentsmesh_types::RunnerStatus::Offline)).unwrap();
        assert_eq!(repo.list_all().unwrap().len(), 2);
    }

    #[test]
    fn get_nonexistent_returns_none() {
        let repo = make_repo();
        assert!(repo.get(999).unwrap().is_none());
    }

    #[test]
    fn delete_nonexistent_is_noop() {
        let repo = make_repo();
        repo.delete(999).unwrap();
    }

    #[test]
    fn save_overwrites_existing() {
        let repo = make_repo();
        repo.save(&make_runner(1, agentsmesh_types::RunnerStatus::Online)).unwrap();
        repo.save(&make_runner(1, agentsmesh_types::RunnerStatus::Offline)).unwrap();
        let loaded = repo.get(1).unwrap().unwrap();
        assert_eq!(loaded.status, agentsmesh_types::RunnerStatus::Offline);
        assert_eq!(repo.list_all().unwrap().len(), 1);
    }

    #[test]
    fn filter_by_status_empty() {
        let repo = make_repo();
        repo.save(&make_runner(1, agentsmesh_types::RunnerStatus::Online)).unwrap();
        assert!(repo.get_by_status("offline").unwrap().is_empty());
    }

    #[test]
    fn delete_removes_from_status_index() {
        let repo = make_repo();
        repo.save(&make_runner(1, agentsmesh_types::RunnerStatus::Online)).unwrap();
        repo.delete(1).unwrap();
        assert!(repo.get_by_status("online").unwrap().is_empty());
    }

    #[test]
    fn list_empty_table() {
        let repo = make_repo();
        assert!(repo.list_all().unwrap().is_empty());
    }
}
