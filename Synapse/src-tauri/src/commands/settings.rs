//! Settings management commands
//! 
//! This module handles all settings-related commands including:
//! - Settings retrieval and updates
//! - API key management
//! - Settings validation

use tauri::State;
use crate::settings::{Settings, SettingsManager, Validate};
use super::{CommandResult, CommandError};

/// Retrieves the current application settings
/// 
/// # Errors
/// Returns an error if the settings cannot be loaded
#[tauri::command]
pub async fn get_settings(
    settings_manager: State<'_, SettingsManager>
) -> CommandResult<Settings> {
    settings_manager
        .get_settings()
        .await
        .map_err(CommandError::from)
}

/// Updates the application settings
/// 
/// # Arguments
/// * `settings` - The new settings to apply
/// 
/// # Errors
/// Returns an error if:
/// - Settings validation fails
/// - Settings cannot be updated
#[tauri::command]
pub async fn update_settings(
    settings: Settings,
    settings_manager: State<'_, SettingsManager>
) -> CommandResult<()> {
    // Validate settings before updating
    settings.validate()
        .map_err(|e| CommandError::InvalidInput(e))?;

    settings_manager
        .update_settings(settings)
        .await
        .map_err(CommandError::from)
}

/// Stores an API key for a specific provider
/// 
/// # Arguments
/// * `provider` - The name of the AI provider (e.g., "openai", "anthropic")
/// * `key` - The API key to store
/// 
/// # Errors
/// Returns an error if:
/// - Provider name is invalid
/// - Key cannot be stored
#[tauri::command]
pub async fn store_api_key(
    provider: String,
    key: String,
    settings_manager: State<'_, SettingsManager>
) -> CommandResult<()> {
    // Validate provider name
    if !["openai", "anthropic"].contains(&provider.as_str()) {
        return Err(CommandError::InvalidInput(format!("Invalid provider: {}", provider)));
    }

    settings_manager
        .store_api_key(&provider, &key)
        .await
        .map_err(CommandError::from)
}

/// Retrieves an API key for a specific provider
/// 
/// # Arguments
/// * `provider` - The name of the AI provider
/// 
/// # Errors
/// Returns an error if:
/// - Provider name is invalid
/// - Key cannot be retrieved
#[tauri::command]
pub async fn get_api_key(
    provider: String,
    settings_manager: State<'_, SettingsManager>
) -> CommandResult<String> {
    // Validate provider name
    if !["openai", "anthropic"].contains(&provider.as_str()) {
        return Err(CommandError::InvalidInput(format!("Invalid provider: {}", provider)));
    }

    settings_manager
        .get_api_key(&provider)
        .await
        .map_err(CommandError::from)
}

/// Deletes an API key for a specific provider
/// 
/// # Arguments
/// * `provider` - The name of the AI provider
/// 
/// # Errors
/// Returns an error if:
/// - Provider name is invalid
/// - Key cannot be deleted
#[tauri::command]
pub async fn delete_api_key(
    provider: String,
    settings_manager: State<'_, SettingsManager>
) -> CommandResult<()> {
    // Validate provider name
    if !["openai", "anthropic"].contains(&provider.as_str()) {
        return Err(CommandError::InvalidInput(format!("Invalid provider: {}", provider)));
    }

    settings_manager
        .delete_api_key(&provider)
        .await
        .map_err(CommandError::from)
} 