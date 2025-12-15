// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    not(any(debug_assertions, feature = "console_log")),
    windows_subsystem = "windows"
)]

// Enable console logging in debug builds
#[cfg(debug_assertions)]
extern crate console_error_panic_hook;

use std::sync::Once;
use tauri::Manager;

static START: Once = Once::new();

#[cfg(debug_assertions)]
fn main() {
    console_error_panic_hook::set_once();
    START.call_once(|| {
        tracing_subscriber::fmt::init();
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_scaffold,
            validate_api_key,
            save_api_key,
            get_api_key,
            create_project,
            validate_project_path,
            select_directory,
            open_in_vscode,
            get_project_history,
            add_to_history,
            delete_from_history,
            open_in_finder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(not(debug_assertions))]
fn main() {
    START.call_once(|| {
        tracing_subscriber::fmt::init();
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_scaffold,
            validate_api_key,
            save_api_key,
            get_api_key,
            create_project,
            validate_project_path,
            select_directory,
            open_in_vscode,
            get_project_history,
            add_to_history,
            delete_from_history,
            open_in_finder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}