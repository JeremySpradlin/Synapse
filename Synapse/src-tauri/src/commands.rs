use crate::settings::{Settings, SettingsManager};
use tauri::{State, Window, PhysicalPosition};

// Window Commands

#[tauri::command]
pub fn get_window_position(window: Window) -> (i32, i32) {
    window.outer_position()
        .map(|pos| (pos.x, pos.y))
        .unwrap_or((0, 0))
}

#[tauri::command]
pub fn set_window_position(window: Window, x: i32, y: i32) {
    let _ = window.set_position(tauri::Position::Physical(
        PhysicalPosition { x, y }
    ));
}

// Settings Commands

#[tauri::command]
pub async fn get_settings(
    settings_manager: State<'_, SettingsManager>
) -> Result<Settings, String> {
    settings_manager
        .get_settings()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_settings(
    settings: Settings,
    settings_manager: State<'_, SettingsManager>
) -> Result<(), String> {
    settings_manager
        .update_settings(settings)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn store_api_key(
    provider: String,
    key: String,
    settings_manager: State<'_, SettingsManager>
) -> Result<(), String> {
    settings_manager
        .store_api_key(&provider, &key)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_api_key(
    provider: String,
    settings_manager: State<'_, SettingsManager>
) -> Result<String, String> {
    settings_manager
        .get_api_key(&provider)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_api_key(
    provider: String,
    settings_manager: State<'_, SettingsManager>
) -> Result<(), String> {
    settings_manager
        .delete_api_key(&provider)
        .await
        .map_err(|e| e.to_string())
} 
