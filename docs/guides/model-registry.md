# Model Registry

The model registry provides a centralized catalog of supported LLM and embedding models with their default configurations.

## Overview

The registry ensures that:
- Embedding models always have their correct embedding dimensions specified
- Context lengths and other model-specific defaults are accurate
- Model metadata is validated at compile time
- Adding new models is straightforward and type-safe

## Using the Registry

```rust
use nucleus_core::models::{ModelRegistry, Model};

let registry = ModelRegistry::new();

// Get a specific embedding model
if let Some(embed) = registry.get_embedding("qwen3-embedding-0.6b") {
    println!("Embedding dimension: {}", embed.embedding_dim);
    println!("Context length: {}", embed.context_length);
    println!("HuggingFace repo: {}", embed.hf_repo);
}

// List all embedding models
for embed in registry.embedding_models() {
    println!("{}: {} dimensions", embed.name, embed.embedding_dim);
}

// List all chat models
for chat in registry.chat_models() {
    println!("{}: {} context", chat.name, chat.context_length);
}
```

## Model Types

### ChatModel

Chat models are general-purpose language models for conversation and text generation.

**Fields:**
- `id`: Unique identifier (e.g., "qwen3-0.6b")
- `name`: Human-readable name
- `context_length`: Maximum token context window
- `default_temperature`: Recommended temperature setting
- `description`: Brief description of the model

### EmbeddingModel

Embedding models generate vector representations of text for semantic search and retrieval.

**Fields:**
- `id`: Unique identifier (e.g., "qwen3-embedding-0.6b")
- `name`: Human-readable name
- `hf_repo`: HuggingFace repository path (e.g., "Qwen/Qwen3-Embedding-0.6B")
- `context_length`: Maximum token context window
- `embedding_dim`: Output vector dimension (critical for vector DB setup)
- `supports_custom_dimensions`: Whether the model supports MRL (variable dimensions)
- `description`: Brief description of the model

## Adding New Models

To add a new model to the registry:

### 1. Research the Model

Before adding a model, gather accurate information:
- Context length from official documentation
- Embedding dimension (for embedding models)
- Default parameters (temperature, etc.)
- HuggingFace repository path
- Whether it supports custom dimensions (MRL)

**Example: Checking HuggingFace**

For embedding models, visit the HuggingFace model page and look for:
- `max_position_embeddings` or "Context Length" in the model card
- `hidden_size` or "Embedding Dimension" 
- Whether "MRL" (Matryoshka Representation Learning) is mentioned

### 2. Add to the Registry

Edit `nucleus-core/src/models/registry.rs` and add your model to the `default_models()` function:

**Adding a Chat Model:**

```rust
pub fn default_models() -> Vec<Model> {
    vec![
        // ... existing models ...
        
        Model::Chat(ChatModel {
            id: "llama3-8b".to_string(),
            name: "Llama 3 8B".to_string(),
            context_length: 8192,
            default_temperature: 0.7,
            description: "General purpose chat model".to_string(),
        }),
    ]
}
```

**Adding an Embedding Model:**

```rust
pub fn default_models() -> Vec<Model> {
    vec![
        // ... existing models ...
        
        Model::Embedding(EmbeddingModel {
            id: "bge-base-en-v1.5".to_string(),
            name: "BGE Base English v1.5".to_string(),
            hf_repo: "BAAI/bge-base-en-v1.5".to_string(),
            context_length: 512,
            embedding_dim: 768,
            supports_custom_dimensions: false,
            description: "Balanced English embeddings with 768 dimensions".to_string(),
        }),
    ]
}
```

### 3. Verify

Run the tests to ensure your model was added correctly:

```bash
cargo test --lib -p nucleus-core models::
```

Run the example to see your model in the registry:

```bash
cargo run --example model_registry
```

## Why Type Safety Matters

The registry uses separate `ChatModel` and `EmbeddingModel` types to enforce correctness:

**Compile-time guarantees:**
- Embedding models MUST specify `embedding_dim` (can't forget it)
- Chat models MUST specify `default_temperature`
- Type system prevents mixing up model types

**Runtime safety:**
- Vector databases can query the correct dimension for models
- Configuration can validate embedding dimensions match the model
- No silent failures from dimension mismatches

## Currently Supported Models

### Embedding Models

- **Qwen3 Embedding 0.6B** (`qwen3-embedding-0.6b`)
  - 1024 dimensions (supports MRL for custom dimensions)
  - 32k context length
  - Multilingual support for 100+ languages

_More models will be added as support is implemented._

## Integration with Vector Databases

When setting up a vector database, use the registry to get the correct embedding dimension:

```rust
use nucleus_core::models::ModelRegistry;

let registry = ModelRegistry::new();
let embed_model = registry.get_embedding("qwen3-embedding-0.6b")
    .expect("Model not found");

// Use the correct dimension for your vector DB
let vector_dim = embed_model.embedding_dim; // 1024

// Initialize your vector DB with this dimension
// vectordb.create_collection("my_collection", vector_dim);
```

This prevents dimension mismatches that would cause runtime errors or incorrect results.
