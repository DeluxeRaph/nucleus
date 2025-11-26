use nucleus_core::{ollama::{ChatRequest, Message}, Config, OllamaClient};

#[tokio::main]
async fn main() {
    println!("Welcome to Nucleus!");

    let config = match Config::load_default() {
        Ok(c) => {
            println!("Config loaded");
            println!("Model: {}", c.llm.model);
            println!("Base URL: {}", c.llm.base_url);
            c
        }
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            eprintln!("Make sure nucleus-core/config.yaml exists\n");
            return
        }
    };

    let model = &config.llm.model;
    let client = OllamaClient::new(&config.llm.base_url);

    println!("Testing chat with model: {}", model);
    let request = ChatRequest::new(model, vec![Message::user("Hi there!")]);

    std::io::Write::flush(&mut std::io::stdout()).ok();

    match client
        .chat(request, |response| {
            print!("{}", response.message.content);
            std::io::Write::flush(&mut std::io::stdout()).ok();
        })
        .await
    {
        Ok(_) => {
            println!("\n\nChat test successful!");
        }
        Err(e) => {
            eprintln!("\n\nChat test failed: {}", e);
            eprintln!("\nTroubleshooting:");
            eprintln!("  1. Is Ollama running? (ollama serve)");
            eprintln!("  2. Is the model pulled? (ollama pull {})", config.llm.model);
            eprintln!("  3. Is the base URL correct? ({})", config.llm.base_url);
        }
    }
}
