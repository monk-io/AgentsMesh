use std::path::PathBuf;
use agentsmesh_auth::storage::PersistentStorage;

pub struct FileStorage {
    dir: PathBuf,
}

impl FileStorage {
    pub fn new(dir: PathBuf) -> Self { Self { dir } }
    fn path(&self, key: &str) -> PathBuf { self.dir.join(format!("{key}.json")) }
}

impl PersistentStorage for FileStorage {
    fn get(&self, key: &str) -> Option<String> {
        std::fs::read_to_string(self.path(key)).ok()
    }
    fn set(&self, key: &str, value: &str) {
        let _ = std::fs::write(self.path(key), value);
    }
    fn remove(&self, key: &str) {
        let _ = std::fs::remove_file(self.path(key));
    }
    fn clear(&self) {
        let _ = std::fs::remove_file(self.path("agentsmesh-auth"));
    }
}
