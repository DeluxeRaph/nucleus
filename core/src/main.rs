mod config;

fn main() {
    match config::Config::load_default() {
        Ok(cfg) => {
            println!("Config loaded successfully!");
            println!("Model: {}", cfg.llm.model);
            println!("Base URL: {}", cfg.llm.base_url);
            println!("Permissions - Read: {}, Write: {}, Command: {}",
                cfg.permission.read, cfg.permission.write, cfg.permission.command);
        }
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    }
}
