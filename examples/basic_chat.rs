use std::sync::Arc;

use nucleus::{ChatManager, Config};
use nucleus_plugin::{Permission, PluginRegistry};



#[tokio::main]
async fn main() {
    let config = Config::load_or_default();
    let registry = PluginRegistry::new(Permission::NONE);
    let manager = ChatManager::new(config, Arc::new(registry)).await.unwrap();

    let message = "Hi!".to_string();
    println!("Message: {}", message);

    let response = manager.query(&message).await;

    println!("Response: {}", response.unwrap());
}
