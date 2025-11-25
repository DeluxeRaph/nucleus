use super::types::{Request, StreamChunk};
use crate::{config::Config, ollama};
use tokio::sync::mpsc;

pub type ChunkSender = mpsc::UnboundedSender<StreamChunk>;

/// Handles different request types and sends responses via channel.
pub struct RequestHandler {
    config: Config,
    ollama_client: ollama::Client,
}

impl RequestHandler {
    pub fn new(config: Config, ollama_client: ollama::Client) -> Self {
        Self {
            config,
            ollama_client,
        }
    }
    
    /// Routes request to appropriate handler based on type.
    pub async fn handle(&self, request: Request, sender: ChunkSender) {
        match request.request_type.as_str() {
            "chat" | "edit" => self.handle_chat(request, sender).await,
            "add" => self.handle_add(request, sender).await,
            "index" => self.handle_index(request, sender).await,
            "stats" => self.handle_stats(sender).await,
            _ => {
                let _ = sender.send(StreamChunk::error("Unknown request type"));
            }
        }
    }
    
    async fn handle_chat(&self, request: Request, sender: ChunkSender) {
        let messages = self.build_messages(request);
        
        let chat_request = ollama::ChatRequest::new(&self.config.llm.model, messages)
            .with_temperature(self.config.llm.temperature);
        
        let mut full_response = String::new();
        
        let result = self.ollama_client.chat(chat_request, |response| {
            if !response.message.content.is_empty() {
                full_response.push_str(&response.message.content);
                let _ = sender.send(StreamChunk::chunk(&response.message.content));
            }
        }).await;
        
        match result {
            Ok(_) => {
                let _ = sender.send(StreamChunk::done(&full_response));
            }
            Err(e) => {
                let _ = sender.send(StreamChunk::error(e.to_string()));
            }
        }
    }
    
    async fn handle_add(&self, request: Request, sender: ChunkSender) {
        let _ = sender.send(StreamChunk::done(format!(
            "Added to knowledge base: {} (RAG not implemented yet)",
            request.content
        )));
    }
    
    async fn handle_index(&self, request: Request, sender: ChunkSender) {
        let _ = sender.send(StreamChunk::done(format!(
            "Indexed directory: {} (RAG not implemented yet)",
            request.content
        )));
    }
    
    async fn handle_stats(&self, sender: ChunkSender) {
        let _ = sender.send(StreamChunk::done(
            "Knowledge base: 0 documents (RAG not implemented yet)"
        ));
    }
    
    fn build_messages(&self, request: Request) -> Vec<ollama::Message> {
        let mut messages = vec![ollama::Message::system(&self.config.system_prompt)];
        
        if let Some(history) = request.history {
            for msg in history {
                messages.push(ollama::Message {
                    role: msg.role,
                    content: msg.content,
                });
            }
        }
        
        messages.push(ollama::Message::user(&request.content));
        messages
    }
}
