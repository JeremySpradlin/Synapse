//! Window management commands
//! 
//! This module handles all window-related commands including:
//! - Window positioning
//! - Window visibility
//! - Settings window management

use tauri::{Window, PhysicalPosition, WindowBuilder, WindowUrl, Manager};
use log::{error, info};
use tokio;

use super::{CommandResult, CommandError};

/// Gets the current window position
/// 
/// Returns a tuple of (x, y) coordinates. If the position cannot be determined,
/// returns (0, 0) as a fallback.
#[tauri::command]
pub fn get_window_position(window: Window) -> CommandResult<(i32, i32)> {
    window.outer_position()
        .map(|pos| (pos.x, pos.y))
        .map_err(|e| CommandError::Window(format!("Failed to get window position: {}", e)))
}

/// Sets the window position to specific coordinates
/// 
/// # Arguments
/// * `window` - The window to move
/// * `x` - The x coordinate
/// * `y` - The y coordinate
/// 
/// # Errors
/// Returns an error if the window position cannot be set
#[tauri::command]
pub fn set_window_position(window: Window, x: i32, y: i32) -> CommandResult<()> {
    window.set_position(tauri::Position::Physical(
        PhysicalPosition { x, y }
    )).map_err(|e| CommandError::Window(format!("Failed to set window position: {}", e)))
}

/// Opens the settings window
/// 
/// Creates a new settings window if one doesn't exist, or focuses the existing one.
/// The window is created with standard decorations and is resizable.
/// 
/// # Arguments
/// * `window` - The main application window
/// 
/// # Errors
/// Returns an error if:
/// - Window creation fails
/// - Window operations (show/focus) fail
/// - Window event setup fails
#[tauri::command]
pub async fn open_settings_window(window: Window) -> CommandResult<()> {
    let app = window.app_handle();
    let main_window = window.clone();
    
    // First, ensure any existing settings window is properly cleaned up
    if let Some(existing_window) = app.get_window("settings") {
        existing_window.close().map_err(|e| 
            CommandError::Window(format!("Failed to close existing settings window: {}", e)))?;
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    // Get the main window's current position and size for relative positioning
    let main_pos = main_window.outer_position()
        .map_err(|e| CommandError::Window(format!("Failed to get main window position: {}", e)))?;
    let main_size = main_window.outer_size()
        .map_err(|e| CommandError::Window(format!("Failed to get main window size: {}", e)))?;

    info!("Creating settings window...");

    // Calculate the center position relative to the main window
    let settings_width = 1400.0;
    let settings_height = 900.0;
    
    let x = main_pos.x + ((main_size.width as i32 - settings_width as i32) / 2);
    let y = main_pos.y + ((main_size.height as i32 - settings_height as i32) / 2);

    info!("Main window position: ({}, {}), size: {}x{}", 
        main_pos.x, main_pos.y, main_size.width, main_size.height);
    info!("Settings window size: {}x{}", settings_width, settings_height);
    info!("Calculated settings window position: ({}, {})", x, y);

    // Create the settings window with explicit positioning
    let settings_window = WindowBuilder::new(
        &app,
        "settings",
        WindowUrl::App("/src/pages/settings.html".into())
    )
    .title("Synapse Settings")
    .inner_size(settings_width, settings_height)
    .position(x as f64, y as f64)
    .visible(false)
    .decorations(true)
    .resizable(true)
    .build()
    .map_err(|e| CommandError::Window(format!("Failed to create settings window: {}", e)))?;

    info!("Created settings window with URL: /src/pages/settings.html");

    // Set up window event handlers
    let settings_window_clone = settings_window.clone();
    let main_window_clone = main_window.clone();
    settings_window.on_window_event(move |event| {
        match event {
            tauri::WindowEvent::CloseRequested { .. } => {
                if let Err(e) = settings_window_clone.close() {
                    error!("Failed to close settings window: {}", e);
                }
                // Show the main window when settings window is closed
                if let Err(e) = main_window_clone.show() {
                    error!("Failed to show main window: {}", e);
                }
            }
            _ => {}
        }
    });

    // Ensure window is in the right position before showing
    settings_window.set_position(tauri::Position::Physical(PhysicalPosition { 
        x: x as i32,
        y: y as i32
    })).map_err(|e| 
        CommandError::Window(format!("Failed to position settings window: {}", e)))?;

    // Now show and focus the window
    settings_window.show().map_err(|e| 
        CommandError::Window(format!("Failed to show settings window: {}", e)))?;
    settings_window.set_focus().map_err(|e| 
        CommandError::Window(format!("Failed to focus settings window: {}", e)))?;

    Ok(())
} 