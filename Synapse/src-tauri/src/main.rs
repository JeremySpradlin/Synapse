// src-tauri/src/main.rs

//! Synapse - A modern application launcher and command palette
//! 
//! This module contains the main application logic for the Synapse launcher,
//! handling window management, global shortcuts, and system integration.

use tauri::{
    GlobalShortcutManager, Manager, PhysicalPosition, Monitor, Window,
    WindowBuilder, WindowUrl,
};
use log::{error, info, warn};

mod settings;
mod commands;

use settings::SettingsManager;
use window_shadows::set_shadow;

/// Error type for window management operations
#[derive(Debug, thiserror::Error)]
pub enum WindowError {
    #[error("Window operation failed: {0}")]
    Operation(String),
    
    #[error("Window not found")]
    NotFound,
    
    #[error("Invalid window state")]
    InvalidState,
}

type WindowResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Centers the window horizontally at the top of the specified monitor
/// 
/// # Arguments
/// * `window` - The window to center
/// * `monitor` - The monitor to center the window on
pub(crate) fn center_window_horizontally(window: &Window, monitor: &Monitor) -> WindowResult<()> {
    let screen_size = monitor.size();
    let window_size = window.outer_size()?;
    
    let x = (screen_size.width as i32 - window_size.width as i32) / 2;
    let y = 0; // Position at the top of the screen
    
    window.set_position(tauri::Position::Physical(PhysicalPosition { x, y }))?;
    Ok(())
}

/// Window management functions for handling window state and transitions
mod window_management {
    use super::*;

    /// Shows the main window and ensures proper state
    pub fn show_main_window(window: &Window) -> WindowResult<()> {
        info!("Showing main window");
        // Ensure proper window position
        if let Ok(Some(monitor)) = window.primary_monitor() {
            center_window_horizontally(window, &monitor)?;
        } else {
            warn!("Could not get primary monitor, window may not be centered");
        }

        // Configure and show window
        window.set_always_on_top(true)?;
        window.show()?;
        window.set_focus()?;
        info!("Main window shown and focused");

        Ok(())
    }

    /// Hides the main window and cleans up state
    pub fn hide_main_window(window: &Window) -> WindowResult<()> {
        info!("Hiding main window");
        window.hide()?;
        info!("Main window hidden");
        Ok(())
    }

    /// Sets up window focus event handlers
    pub fn setup_window_focus_handlers(window: &Window) -> WindowResult<()> {
        info!("Setting up window focus handlers");
        let win = window.clone();
        
        window.on_window_event(move |event| {
            match event {
                tauri::WindowEvent::Focused(focused) => {
                    if !focused {
                        info!("Window lost focus, hiding");
                        if let Err(e) = hide_main_window(&win) {
                            error!("Failed to hide window on focus loss: {}", e);
                        }
                    }
                }
                _ => {}
            }
        });

        Ok(())
    }

    /// Handles transitioning between main and settings windows
    pub fn handle_settings_window(window: &Window) -> WindowResult<()> {
        info!("Opening settings window");
        let main_window = window.clone();
        
        // Create settings window
        let settings = WindowBuilder::new(
            &window.app_handle(),
            "settings",
            WindowUrl::App("settings.html".into())
        )
        .title("Settings")
        .inner_size(600.0, 400.0)
        .center()
        .decorations(true)
        .always_on_top(false)
        .build()?;

        // Handle settings window closure
        settings.on_window_event(move |event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                info!("Settings window closing, restoring main window");
                if let Err(e) = show_main_window(&main_window) {
                    error!("Failed to restore main window: {}", e);
                }
            }
        });

        // Show settings window
        settings.show()?;
        settings.set_focus()?;
        info!("Settings window shown and focused");

        // Hide main window
        hide_main_window(window)?;

        Ok(())
    }
}

/// Sets up the global shortcut for toggling window visibility
fn setup_global_shortcut(app: &tauri::App) -> WindowResult<()> {
    let window = app.get_window("main")
        .ok_or("Failed to get main window")?;
    let mut shortcut_manager = app.global_shortcut_manager();
    
    info!("Registering global shortcut (CommandOrControl+Shift+Space)");
    shortcut_manager.register("CommandOrControl+Shift+Space", move || {
        info!("Global shortcut triggered");
        
        if let Ok(is_visible) = window.is_visible() {
            info!("Window visibility state: {}", is_visible);
            
            let result = if is_visible {
                window_management::hide_main_window(&window)
            } else {
                window_management::show_main_window(&window)
            };

            if let Err(e) = result {
                error!("Failed to toggle window visibility: {}", e);
            }
        }
    })?;
    
    info!("Global shortcut registered successfully");
    Ok(())
}

/// Initializes the application window
fn setup_window(window: &Window) -> WindowResult<()> {
    // Configure window appearance
    window.set_decorations(false)?;
    window.set_always_on_top(true)?;
    window.hide()?;

    // Center window if possible
    if let Some(monitor) = window.primary_monitor()? {
        center_window_horizontally(window, &monitor)?;
    }

    // Listen for settings window events
    let window_clone = window.clone();
    window.listen("open_settings", move |_| {
        if let Err(e) = window_management::handle_settings_window(&window_clone) {
            eprintln!("Failed to handle settings window: {}", e);
        }
    });

    Ok(())
}

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();
    info!("Initializing Synapse application");

    let settings_manager = SettingsManager::new()
        .await
        .expect("Failed to initialize settings manager");

    tauri::Builder::default()
        .manage(settings_manager)
        .invoke_handler(tauri::generate_handler![
            commands::get_window_position,
            commands::set_window_position,
            commands::get_settings,
            commands::update_settings,
            commands::store_api_key,
            commands::get_api_key,
            commands::delete_api_key,
        ])
        .setup(|app| {
            info!("Setting up main application window");
            let window = app.get_window("main")
                .ok_or("Failed to get main window")?;
            
            // Initialize window systems
            window_management::setup_window_focus_handlers(&window)?;
            setup_window(&window)?;
            setup_global_shortcut(app)?;
            
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&window, true).expect("Failed to set window shadow");
            
            info!("Application initialization complete");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
