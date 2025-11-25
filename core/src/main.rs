mod config;
mod ollama;

#[tokio::main]
async fn main() {
    let cfg = match config::Config::load_default() {
        Ok(c) => {
            println!("✓ Config loaded");
            println!("  Model: {}", c.llm.model);
            println!("  Base URL: {}", c.llm.base_url);
            c
        }
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    println!("\n--- Testing Ollama Client ---\n");
    
    let client = ollama::Client::new(&cfg.llm.base_url);
    
    let messages = vec![
        ollama::Message::system("You are a helpful assistant."),
        ollama::Message::user("Say 'Hello from Rust!' in one sentence."),
    ];
    
    let request = ollama::ChatRequest::new(&cfg.llm.model, messages)
        .with_temperature(cfg.llm.temperature);
    
    println!("Streaming response:");
    let mut full_response = String::new();
    
    match client.chat(request, |response| {
        if !response.message.content.is_empty() {
            print!("{}", response.message.content);
            full_response.push_str(&response.message.content);
        }
    }).await {
        Ok(_) => {
            println!("\n\n✓ Chat completed");
            println!("Full response length: {} chars", full_response.len());
        }
        Err(e) => {
            eprintln!("\n✗ Chat failed: {}", e);
        }
    }
}
