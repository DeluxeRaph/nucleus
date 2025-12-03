use std::io::{self, Write};
use std::sync::Arc;

use nucleus::{ChatManager, Config};
use nucleus_plugin::{Permission, PluginRegistry};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("nucleus_core=info".parse().unwrap())
        )
        .init();

    let config = Config::load_or_default();
    let registry = Arc::new(PluginRegistry::new(Permission::NONE));
    let manager = ChatManager::new(config, registry)
        .await
        .expect("Failed to create chat manager");

    let message = "Write me a short poem about Rust programming";
    println!("User: {}", message);
    println!("\nAssistant: ");
    io::stdout().flush().unwrap();

    // Stream response with live printing
    let response = manager
        .query_stream(message, |chunk| {
            print!("{}", chunk);
            io::stdout().flush().unwrap();
        })
        .await
        .expect("Failed to get response");

    println!("\n\n--- Stream complete ---");
    println!("Total length: {} characters", response.len());
}
