//! mistral.rs provider implementation.
//!
//! This module provides an in-process LLM provider using mistral.rs.
//! Models are loaded from disk and run locally.

use super::types::*;
use async_trait::async_trait;


pub struct MistralRsProvider {
    model_name: String,
}

impl MistralRsProvider {
    /// Creates a new mistral.rs provider.
    pub fn new(model_name: impl Into<String>) -> Self {
        Self {
            model_name: model_name.into(),
        }
    }
}

impl Default for MistralRsProvider {
    fn default() -> Self {
        Self::new("mistral")
    }
}

#[async_trait]
impl Provider for MistralRsProvider {
    async fn chat<'a>(
        &'a self,
        request: ChatRequest,
        mut callback: Box<dyn FnMut(ChatResponse) + Send + 'a>,
    ) -> Result<()> {


        Err(ProviderError::Other(
            format!(
                "MistralRsProvider not yet fully implemented.\n\n\
                To use mistral.rs:\n\
                1. Download a model (GGUF format recommended)\n\
                2. Implement model loading in this provider\n\
                3. Configure model path in your config\n\n\
                Requested model: {}\n\n\
                For now, use OllamaProvider instead.",
                request.model
            )
        ))
    }

    async fn embed(&self, _text: &str, _model: &str) -> Result<Vec<f32>> {
        Err(ProviderError::Other(
            "Embeddings not yet supported for mistral.rs provider".to_string(),
        ))
    }
}
