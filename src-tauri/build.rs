fn main() {
    use std::env;

    if env::var("DOCS_RS").is_ok() {
        return;
    }

    tauri_build::build()
}
