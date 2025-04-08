use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestConfig {
    pub bytes: usize,
    pub kilobytes: usize,
    pub megabytes: usize,
    pub total_messages: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub tests: TestConfig,
    pub ip: String,
    pub port: u16,
}

impl TestConfig {
    // Méthode utilitaire pour calculer la taille totale en bytes
    pub fn get_total_size(&self) -> usize {
        self.bytes + (self.kilobytes * 1024) + (self.megabytes * 1024 * 1024)
    }
}
