use std::path::Path;

pub fn is_local_gguf(model: &str) -> bool {
    model.ends_with(".gguf") && Path::new(&model).exists()
}
