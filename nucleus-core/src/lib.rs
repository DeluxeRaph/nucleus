//! nucleus-core - Core AI engine infrastructure
//!
//! Provides the foundational components for building AI-powered applications:
//! - LLM integration (Ollama client)
//! - RAG (Retrieval Augmented Generation)
//! - Configuration management
//! - HTTP/Unix socket server

pub mod config;
pub mod ollama;
pub mod rag;
pub mod server;

// Re-export commonly used types
pub use config::Config;
pub use ollama::Client as OllamaClient;
pub use server::Server;
