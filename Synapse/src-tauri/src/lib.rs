//! Synapse library crate
//! 
//! This module serves as the library entry point for the Synapse application.
//! It provides the core functionality that can be shared between different
//! entry points (desktop, mobile, etc.).

use tauri::Builder;
use commands::window::{get_window_position, set_window_position, open_settings_window};
use commands::settings::{get_settings, update_settings, store_api_key, get_api_key, delete_api_key};

pub mod commands;
pub mod settings;
pub mod services;
pub mod utils;

#[cfg(mobile)]
mod mobile;

#[cfg(mobile)]
pub use mobile::*;

/// Initialize the core application builder with common configuration
pub async fn create_app() -> Builder<tauri::Wry> {
    let settings_manager = settings::SettingsManager::new()
        .await
        .expect("Failed to initialize settings manager");

    Builder::default()
        .manage(settings_manager)
        .invoke_handler(tauri::generate_handler![
            // Window commands
            get_window_position,
            set_window_position,
            open_settings_window,
            
            // Settings commands
            get_settings,
            update_settings,
            store_api_key,
            get_api_key,
            delete_api_key,
        ])
}
