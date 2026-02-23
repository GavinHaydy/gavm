// main.rs
// Entry point of Tauri application

mod commands;
mod core;

use commands::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_versions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}