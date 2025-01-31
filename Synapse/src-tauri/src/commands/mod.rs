//! Command handlers for Tauri frontend integration
//! 
//! This module contains all command handlers that can be invoked from the frontend,
//! organized by feature area.

use serde::Serialize;
use thiserror::Error;

pub mod window;
pub mod settings;

// Re-export all commands with their Tauri command attributes
pub use window::{
    get_window_position,
    set_window_position,
    open_settings_window,
};

pub use settings::{
    get_settings,
    update_settings,
    store_api_key,
    get_api_key,
    delete_api_key,
};

/// Error type for command handlers
#[derive(Debug, Error, Serialize)]
pub enum CommandError {
    #[error("Window operation failed: {0}")]
    Window(String),
    
    #[error("Settings operation failed: {0}")]
    Settings(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

/// Result type alias for command handlers
pub type CommandResult<T> = Result<T, CommandError>;

impl From<crate::utils::AppError> for CommandError {
    fn from(error: crate::utils::AppError) -> Self {
        match error {
            crate::utils::AppError::InvalidInput(msg) => CommandError::InvalidInput(msg),
            crate::utils::AppError::Internal(msg) => CommandError::Internal(msg),
            _ => CommandError::Internal(error.to_string()),
        }
    }
}

impl From<crate::settings::SettingsError> for CommandError {
    fn from(error: crate::settings::SettingsError) -> Self {
        CommandError::Settings(error.to_string())
    }
} 