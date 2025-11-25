use serde::{Deserialize, Serialize};

/// A message in conversation history.
///
/// **Note:** This may be identical to the `ollama::Message`.
/// This is kept separate for readability and future modifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Request from client to server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    #[serde(rename = "type")]
    pub request_type: String,
    pub content: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pwd: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<Message>>,
}

/// Streaming response chunk sent to client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    #[serde(rename = "type")]
    pub chunk_type: String,
    pub content: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl StreamChunk {
    pub fn chunk(content: impl Into<String>) -> Self {
        Self {
            chunk_type: "chunk".to_string(),
            content: content.into(),
            error: None,
        }
    }

    pub fn done(content: impl Into<String>) -> Self {
        Self {
            chunk_type: "done".to_string(),
            content: content.into(),
            error: None,
        }
    }

    pub fn error(error: impl Into<String>) -> Self {
        Self {
            chunk_type: "error".to_string(),
            content: String::new(),
            error: Some(error.into()),
        }
    }
}
