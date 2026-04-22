use std::sync::Arc;

use agentsmesh_persistence::{LoopRepo, StorageBackend};
use agentsmesh_types::{LoopData, LoopRunData, LoopRunStatus};

pub struct LoopState {
    loops: Vec<LoopData>,
    current_loop: Option<LoopData>,
    runs: Vec<LoopRunData>,
    repo: Option<LoopRepo>,
}

impl LoopState {
    pub fn new() -> Self {
        Self { loops: Vec::new(), current_loop: None, runs: Vec::new(), repo: None }
    }

    pub fn with_storage(backend: Arc<dyn StorageBackend>) -> Self {
        let repo = LoopRepo::new(backend);
        let loops = repo.list_loops().unwrap_or_default();
        Self { loops, current_loop: None, runs: Vec::new(), repo: Some(repo) }
    }

    pub fn get_loops(&self) -> &[LoopData] { &self.loops }
    pub fn get_current_loop(&self) -> Option<&LoopData> { self.current_loop.as_ref() }
    pub fn get_runs(&self) -> &[LoopRunData] { &self.runs }

    pub fn get_loop_by_slug(&self, slug: &str) -> Option<&LoopData> {
        self.loops.iter().find(|l| l.slug == slug)
    }

    pub fn set_loops(&mut self, loops: Vec<LoopData>) {
        self.loops = loops;
        if let Some(repo) = &self.repo {
            for l in &self.loops { let _ = repo.save_loop(l); }
        }
    }

    pub fn set_current_loop(&mut self, loop_data: Option<LoopData>) {
        self.current_loop = loop_data;
    }

    pub fn update_loop(&mut self, slug: &str, loop_data: LoopData) {
        if let Some(l) = self.loops.iter_mut().find(|l| l.slug == slug) {
            *l = loop_data.clone();
            if let Some(repo) = &self.repo { let _ = repo.save_loop(l); }
        }
        if self.current_loop.as_ref().is_some_and(|l| l.slug == slug) {
            self.current_loop = Some(loop_data);
        }
    }

    pub fn add_run(&mut self, run: LoopRunData) {
        if let Some(repo) = &self.repo { let _ = repo.save_run(&run); }
        self.runs.push(run);
    }

    pub fn set_runs(&mut self, runs: Vec<LoopRunData>) {
        self.runs = runs;
    }

    pub fn append_runs(&mut self, runs: Vec<LoopRunData>) {
        self.runs.extend(runs);
    }

    pub fn update_run_status(&mut self, run_id: i64, status: LoopRunStatus) {
        if let Some(run) = self.runs.iter_mut().find(|r| r.id == run_id) {
            run.status = status;
            if let Some(repo) = &self.repo { let _ = repo.save_run(run); }
        }
    }

    pub fn clear_runs(&mut self) {
        self.runs.clear();
    }
}

impl Default for LoopState {
    fn default() -> Self { Self::new() }
}
