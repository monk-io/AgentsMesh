use std::sync::Arc;

use agentsmesh_types::{LoopData, LoopRunData};

use crate::backend::StorageBackend;
use crate::error::Result;

pub struct LoopRepo {
    backend: Arc<dyn StorageBackend>,
}

impl LoopRepo {
    pub fn new(backend: Arc<dyn StorageBackend>) -> Self {
        Self { backend }
    }

    pub fn save_loop(&self, data: &LoopData) -> Result<()> {
        let bytes = serde_json::to_vec(data)?;
        self.backend.put_raw("loops", &data.slug, &[], &bytes)
    }

    pub fn get_loop(&self, slug: &str) -> Result<Option<LoopData>> {
        match self.backend.get_raw("loops", slug)? {
            Some(data) => Ok(Some(serde_json::from_slice(&data)?)),
            None => Ok(None),
        }
    }

    pub fn delete_loop(&self, slug: &str) -> Result<()> {
        self.backend.delete_raw("loops", slug)
    }

    pub fn list_loops(&self) -> Result<Vec<LoopData>> {
        super::deserialize_rows(self.backend.list_raw("loops")?)
    }

    pub fn save_run(&self, run: &LoopRunData) -> Result<()> {
        let data = serde_json::to_vec(run)?;
        let fields: &[(&str, &str)] = &[("loop_slug", run.loop_slug.as_str())];
        self.backend
            .put_raw("loop_runs", &run.id.to_string(), fields, &data)
    }

    pub fn get_run(&self, id: i64) -> Result<Option<LoopRunData>> {
        match self.backend.get_raw("loop_runs", &id.to_string())? {
            Some(data) => Ok(Some(serde_json::from_slice(&data)?)),
            None => Ok(None),
        }
    }

    pub fn get_runs_by_loop(&self, loop_slug: &str) -> Result<Vec<LoopRunData>> {
        let rows = self
            .backend
            .query_raw("loop_runs", "loop_slug", loop_slug)?;
        let mut runs: Vec<LoopRunData> = super::deserialize_rows(rows)?;
        runs.sort_by(|a, b| b.id.cmp(&a.id));
        Ok(runs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::InMemoryBackend;

    fn make_repo() -> LoopRepo {
        LoopRepo::new(Arc::new(InMemoryBackend::new()))
    }

    #[test]
    fn loop_crud() {
        let repo = make_repo();
        let ld = LoopData {
            slug: "loop-1".into(),
            name: "Hourly".into(),
            description: None,
            schedule: Some("0 * * * *".into()),
            is_enabled: true,
            last_run_at: None,
            created_at: None,
            updated_at: None,
        };
        repo.save_loop(&ld).unwrap();
        let loaded = repo.get_loop("loop-1").unwrap().unwrap();
        assert_eq!(loaded.name, "Hourly");
        repo.delete_loop("loop-1").unwrap();
        assert!(repo.get_loop("loop-1").unwrap().is_none());
    }

    #[test]
    fn list_loops() {
        let repo = make_repo();
        let ld = LoopData {
            slug: "l".into(),
            name: "n".into(),
            description: None,
            schedule: None,
            is_enabled: false,
            last_run_at: None,
            created_at: None,
            updated_at: None,
        };
        repo.save_loop(&ld).unwrap();
        assert_eq!(repo.list_loops().unwrap().len(), 1);
    }

    fn make_run(id: i64, loop_slug: &str) -> LoopRunData {
        LoopRunData {
            id,
            loop_slug: loop_slug.into(),
            status: agentsmesh_types::LoopRunStatus::Completed,
            started_at: None,
            completed_at: None,
            error_message: None,
        }
    }

    #[test]
    fn runs_by_loop_sorted_desc() {
        let repo = make_repo();
        for i in 1..=3 {
            repo.save_run(&make_run(i, "loop-1")).unwrap();
        }
        let runs = repo.get_runs_by_loop("loop-1").unwrap();
        assert_eq!(runs.len(), 3);
        assert!(runs[0].id > runs[1].id);
    }

    #[test]
    fn runs_filtered_by_loop_slug() {
        let repo = make_repo();
        repo.save_run(&make_run(1, "loop-1")).unwrap();
        repo.save_run(&make_run(2, "loop-2")).unwrap();
        assert_eq!(repo.get_runs_by_loop("loop-1").unwrap().len(), 1);
        assert_eq!(repo.get_runs_by_loop("loop-2").unwrap().len(), 1);
    }

    #[test]
    fn get_loop_nonexistent() {
        let repo = make_repo();
        assert!(repo.get_loop("nope").unwrap().is_none());
    }

    #[test]
    fn get_run_nonexistent() {
        let repo = make_repo();
        assert!(repo.get_run(999).unwrap().is_none());
    }

    #[test]
    fn get_run_roundtrip() {
        let repo = make_repo();
        repo.save_run(&make_run(42, "loop-x")).unwrap();
        let loaded = repo.get_run(42).unwrap().unwrap();
        assert_eq!(loaded.loop_slug, "loop-x");
        assert_eq!(loaded.status, agentsmesh_types::LoopRunStatus::Completed);
    }

    #[test]
    fn delete_loop_nonexistent_is_noop() {
        let repo = make_repo();
        repo.delete_loop("nope").unwrap();
    }

    #[test]
    fn save_loop_overwrites() {
        let repo = make_repo();
        let mut ld = LoopData {
            slug: "l1".into(), name: "A".into(), description: None,
            schedule: None, is_enabled: true, last_run_at: None,
            created_at: None, updated_at: None,
        };
        repo.save_loop(&ld).unwrap();
        ld.name = "B".into();
        repo.save_loop(&ld).unwrap();
        let loaded = repo.get_loop("l1").unwrap().unwrap();
        assert_eq!(loaded.name, "B");
        assert_eq!(repo.list_loops().unwrap().len(), 1);
    }

    #[test]
    fn runs_empty_for_unknown_loop() {
        let repo = make_repo();
        assert!(repo.get_runs_by_loop("unknown").unwrap().is_empty());
    }
}
