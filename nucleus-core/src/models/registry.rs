use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatModel {
    pub id: String,
    pub name: String,
    pub context_length: usize,
    pub default_temperature: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingModel {
    pub id: String,
    pub name: String,
    pub path: Option<PathBuf>,
    pub hf_repo: Option<String>,
    pub context_length: usize,
    pub embedding_dim: usize,
    pub description: String,
}

impl Default for EmbeddingModel {
    fn default() -> Self {
        EmbeddingModel {
            id: "qwen3-embedding-0.6b".to_string(),
            name: "Qwen3 Embedding 0.6B".to_string(),
            path: None,
            hf_repo: Some("Qwen/Qwen3-Embedding-0.6B".to_string()),
            context_length: 32768,
            embedding_dim: 1024,
            description: "Multilingual text embedding model with MRL support".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Model {
    Chat(ChatModel),
    Embedding(EmbeddingModel),
}

impl Model {
    pub fn id(&self) -> &str {
        match self {
            Model::Chat(m) => &m.id,
            Model::Embedding(m) => &m.id,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Model::Chat(m) => &m.name,
            Model::Embedding(m) => &m.name,
        }
    }

    pub fn context_length(&self) -> usize {
        match self {
            Model::Chat(m) => m.context_length,
            Model::Embedding(m) => m.context_length,
        }
    }
}

pub struct ModelRegistry {
    models: Vec<Model>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self {
            models: default_models(),
        }
    }

    pub fn get(&self, id: &str) -> Option<&Model> {
        self.models.iter().find(|m| m.id() == id)
    }

    pub fn get_embedding(&self, id: &str) -> Option<&EmbeddingModel> {
        self.get(id).and_then(|m| match m {
            Model::Embedding(embed) => Some(embed),
            _ => None,
        })
    }

    pub fn get_chat(&self, id: &str) -> Option<&ChatModel> {
        self.get(id).and_then(|m| match m {
            Model::Chat(chat) => Some(chat),
            _ => None,
        })
    }

    pub fn chat_models(&self) -> impl Iterator<Item = &ChatModel> {
        self.models.iter().filter_map(|m| match m {
            Model::Chat(chat) => Some(chat),
            _ => None,
        })
    }

    pub fn embedding_models(&self) -> impl Iterator<Item = &EmbeddingModel> {
        self.models.iter().filter_map(|m| match m {
            Model::Embedding(embed) => Some(embed),
            _ => None,
        })
    }

    pub fn all_models(&self) -> &[Model] {
        &self.models
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub fn default_models() -> Vec<Model> {
    vec![Model::Embedding(EmbeddingModel {
        id: "qwen3-embedding-0.6b".to_string(),
        name: "Qwen3 Embedding 0.6B".to_string(),
        path: None,
        hf_repo: Some("Qwen/Qwen3-Embedding-0.6B".to_string()),
        context_length: 32768,
        embedding_dim: 1024,
        description: "Multilingual text embedding model with MRL support".to_string(),
    })]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_has_models() {
        let registry = ModelRegistry::new();
        assert!(!registry.models.is_empty());
    }

    #[test]
    fn test_get_embedding_model() {
        let registry = ModelRegistry::new();
        let model = registry.get("qwen3-embedding-0.6b").unwrap();
        match model {
            Model::Embedding(embed) => {
                assert_eq!(embed.embedding_dim, 1024);
                assert_eq!(embed.context_length, 32768);
                assert_eq!(embed.hf_repo, Some("Qwen/Qwen3-Embedding-0.6B".to_string()));
            }
            _ => panic!("Expected embedding model"),
        }
    }

    #[test]
    fn test_get_embedding_directly() {
        let registry = ModelRegistry::new();
        let embed = registry.get_embedding("qwen3-embedding-0.6b").unwrap();
        assert_eq!(embed.embedding_dim, 1024);
    }

    #[test]
    fn test_all_embedding_models_have_dimensions() {
        let registry = ModelRegistry::new();
        let embeddings: Vec<_> = registry.embedding_models().collect();
        assert!(!embeddings.is_empty());
        for embed in embeddings {
            assert!(embed.embedding_dim > 0);
        }
    }

    #[test]
    fn test_model_ids_unique() {
        let registry = ModelRegistry::new();
        let ids: Vec<_> = registry.all_models().iter().map(|m| m.id()).collect();
        let mut unique_ids = ids.clone();
        unique_ids.sort();
        unique_ids.dedup();
        assert_eq!(ids.len(), unique_ids.len(), "Model IDs must be unique");
    }
}
