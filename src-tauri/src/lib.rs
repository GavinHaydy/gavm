// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod commands;
mod core;

use core::*;
use tauri::Manager;
use utils::config::*;

use commands::*;

use shim::install_shims;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn init_shims() {
    if let Err(e) = install_shims() {
        println!("Error in installing Tauri application: {}", e);
    } else {
        println!("Success!");
    }
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            init_shims();
            init_settings();
            let _app_handle = app.app_handle();
            Ok(())
        })
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            list_versions,
            install,
            base_path,
            get_config_value,
            get_config_values
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
