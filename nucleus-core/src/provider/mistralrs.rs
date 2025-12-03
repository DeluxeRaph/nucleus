//! mistral.rs provider implementation.
//!
//! This module provides an in-process LLM provider using mistral.rs.
//! Supports both local GGUF files and automatic HuggingFace downloads.

use super::{types::*, utils::is_local_gguf};
use async_trait::async_trait;
use mistralrs::{
    AnyMoeLoader, GgufLoraModelBuilder, GgufModelBuilder, GgufXLoraModelBuilder, IsqType, LoraModelBuilder, Model, PagedAttentionMetaBuilder, TextMessageRole, TextMessages, TextModelBuilder, VisionModelBuilder, XLoraModelBuilder
};
use std::path::Path;

/// mistral.rs in-process provider.
///
/// Automatically detects if model is:
/// 1. A local GGUF file path (loads directly)
/// 2. A HuggingFace model ID (downloads if needed)
///
/// Matches OllamaProvider API for easy swapping.
pub struct MistralRsProvider {
    builder: Model,
    model_name: String,
}

impl MistralRsProvider {
    /// Creates a new mistral.rs provider.
    ///
    /// # Model Resolution
    ///
    /// - If `model_name` ends with `.gguf`, treats it as a local file path
    /// - Otherwise, treats it as a HuggingFace model ID (auto-downloads)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // HuggingFace model (auto-downloads)
    /// let provider = MistralRsProvider::new("Qwen/Qwen3-0.6B-Instruct");
    ///
    /// // Local GGUF file
    /// let provider = MistralRsProvider::new("./models/qwen3-0.6b.gguf");
    /// ```
    pub async fn new(model_name: String) -> Result<Self> {
        let builder = Self::build_model(&model_name).await.map_err(|e| ProviderError::Other(format!("Unable to build model: {:?}", e)))?;

        let instance = Self {
            builder: builder,
            model_name: model_name.into(),
        };

        Ok(instance)
    }

    async fn build_model(model_name: &str) -> Result<Model> {
        let model = if is_local_gguf(&model_name) {
            // Extract path and filename to load modal
            let path = Path::new(&model_name);
            let dir = path.parent()
                .ok_or_else(|| ProviderError::Other("Invalid GGUF file path".to_string()))?
                .to_str()
                .ok_or_else(|| ProviderError::Other("Invalid UTF-8 in path".to_string()))?;
            let filename = path.file_name()
                .ok_or_else(|| ProviderError::Other("Invalid GGUF filename".to_string()))?
                .to_str()
                .ok_or_else(|| ProviderError::Other("Invalid UTF-8 in filename".to_string()))?;

            GgufModelBuilder::new(dir, vec![filename])
                .with_logging()
                .with_paged_attn(|| PagedAttentionMetaBuilder::default().build())
                .map_err(|e| ProviderError::Other(format!("Failed to configure paged attention: {:?}", e)))?
                .build()
                .await
                .map_err(|e| ProviderError::Other(format!("Failed to load local GGUF '{}': {:?}", model_name, e)))?
        } else {
            // Download from HuggingFace if not cached  
            TextModelBuilder::new(&model_name)
                .with_isq(IsqType::Q4K) // 4-bit quantization
                .with_logging()
                .with_paged_attn(|| PagedAttentionMetaBuilder::default().build())
                .map_err(|e| ProviderError::Other(format!("Failed to configure paged attention: {:?}", e)))?
                .build()
                .await
                .map_err(|e| ProviderError::Other(
                    format!("Failed to load model '{}'. Make sure it exists on HuggingFace or is a valid local .gguf file: {:?}", 
                        model_name, e)
                ))?
        };

        Ok(model)
    }

    
}

impl Default for MistralRsProvider {
    fn default() -> Self {
        // Default to qwen3 0.6B - small, fast, good quality
        todo!()
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
