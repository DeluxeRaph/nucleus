use nucleus_plugin::{Plugin, PluginError, PluginOutput, Permission, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;
use std::path::PathBuf;

/// Plugin for reading file contents.
pub struct ReadFilePlugin;

#[derive(Debug, Deserialize)]
struct ReadFileParams {
    path: String,
}

impl ReadFilePlugin {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Plugin for ReadFilePlugin {
    fn name(&self) -> &str {
        "read_file"
    }
    
    fn description(&self) -> &str {
        "Read the contents of a file"
    }
    
    fn parameter_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "required": ["path"],
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute or relative path to the file to read"
                }
            }
        })
    }
    
    fn required_permission(&self) -> Permission {
        Permission::READ_ONLY
    }
    
    async fn execute(&self, input: Value) -> Result<PluginOutput> {
        let params: ReadFileParams = serde_json::from_value(input)
            .map_err(|e| PluginError::InvalidInput(format!("Invalid parameters: {}", e)))?;
        
        let path = PathBuf::from(&params.path);
        
        // Read file
        let content = tokio::fs::read_to_string(&path)
            .await
            .map_err(|e| PluginError::ExecutionFailed(format!("Failed to read file: {}", e)))?;
        
        // Log the operation
        println!("Read file: {}", path.display());
        
        Ok(PluginOutput::new(content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_read_file() {
        // Create a temporary file
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("nucleus_test_read.txt");
        let test_content = "Hello, nucleus!";
        
        std::fs::write(&test_file, test_content).unwrap();
        
        // Test reading it
        let plugin = ReadFilePlugin::new();
        let input = serde_json::json!({
            "path": test_file.to_str().unwrap()
        });
        
        let result = plugin.execute(input).await.unwrap();
        assert_eq!(result.content, test_content);
        
        // Cleanup
        std::fs::remove_file(test_file).ok();
    }
    
    #[tokio::test]
    async fn test_read_nonexistent_file() {
        let plugin = ReadFilePlugin::new();
        let input = serde_json::json!({
            "path": "/nonexistent/file.txt"
        });
        
        let result = plugin.execute(input).await;
        assert!(result.is_err());
    }
}
