use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("Failed to parse config: {0}")]
    Parse(#[from] serde_yaml::Error),
}

pub type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub llm: LlmConfig,
    pub system_prompt: String,
    pub rag: RagConfig,
    pub storage: StorageConfig,
    pub personalization: PersonalizationConfig,

    #[serde(skip)]
    pub permission: Permission,
}

/// Permissions granted to the AI.
///
/// **Note**: A permission granted here does not mean it will automatically perform the actions.
/// However, if false, the functionality will not exist to begin with.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Permission {
    /// Read directories and files
    pub read: bool,
    /// Write to files
    pub write: bool,
    /// Run system commands
    pub command: bool,
}

impl Default for Permission {
    fn default() -> Self {
        Self {
            read: true,
            write: true,
            command: true,
        }
    }
}

/// Configuration for the AI model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub model: String,
    pub base_url: String,
    pub temperature: f64,
    pub context_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagConfig {
    pub embedding_model: String,
    pub chunk_size: usize,
    pub chunk_overlap: usize,
    pub top_k: usize,
}
impl Default for RagConfig {
    fn default() -> Self {
        Self {
            embedding_model: "nomic-embed-text".to_string(),
            chunk_size: 512,
            chunk_overlap: 50,
            top_k: 5,
        }
    }
}

/// Vector database for RAG system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub vector_db_path: String,
    pub chat_history_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalizationConfig {
    pub learn_from_interactions: bool,
    pub save_conversations: bool,
    pub user_preferences_path: String,
}
impl Default for PersonalizationConfig {
    fn default() -> Self {
        Self {
            learn_from_interactions: true,
            save_conversations: true,
            user_preferences_path: "./data/preferences.json".to_string(),
        }
    }
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let mut config: Config = serde_yaml::from_str(&contents)?;

        config.permission = Permission::default();
        config.rag = RagConfig::default();
        config.personalization = PersonalizationConfig::default();

        Ok(config)
    }

    pub fn load_default() -> Result<Self> {
        Self::load("config.yaml")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_default() {
        let perm = Permission::default();
        assert!(perm.read);
        assert!(perm.write);
        assert!(perm.command);
    }
}
