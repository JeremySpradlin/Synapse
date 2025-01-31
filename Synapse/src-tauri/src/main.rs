// src-tauri/src/main.rs

//! Synapse - A modern application launcher and command palette
//! 
//! This module contains the main application logic for the Synapse launcher,
//! handling window management, global shortcuts, and system integration.

use tauri::{
    GlobalShortcutManager, Manager, PhysicalPosition, Monitor, Window,
    WindowBuilder, WindowUrl, generate_context,
};
use log::{error, info, warn};
use window_shadows::set_shadow;

/// Error types for window management operations
#[derive(Debug, thiserror::Error)]
pub enum WindowError {
    #[error("Window operation failed: {0}")]
    Operation(String),
    
    #[error("Window not found")]
    NotFound,
    
    #[error("Invalid window state")]
    InvalidState,

    #[error("Monitor not found")]
    MonitorNotFound,
}

type WindowResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Window management module handling all window-related operations
mod window_management {
    use super::*;

    /// Centers the window horizontally at the top of the specified monitor
    pub(crate) fn center_window_horizontally(window: &Window, monitor: &Monitor) -> WindowResult<()> {
        let screen_size = monitor.size();
        let window_size = window.outer_size()?;
        
        let x = (screen_size.width as i32 - window_size.width as i32) / 2;
        let y = 0; // Position at the top of the screen
        
        window.set_position(tauri::Position::Physical(PhysicalPosition { x, y }))?;
        Ok(())
    }

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

    /// Initializes a new window with default configuration
    pub fn setup_window(window: &Window) -> WindowResult<()> {
        info!("Setting up window configuration");
        
        // Configure window appearance
        window.set_decorations(false)?;
        window.set_always_on_top(true)?;
        window.hide()?;

        // Center window if possible
        if let Some(monitor) = window.primary_monitor()? {
            center_window_horizontally(window, &monitor)?;
        }

        Ok(())
    }
}

/// Sets up the global shortcut for toggling window visibility
fn setup_global_shortcut(app: &tauri::App) -> WindowResult<()> {
    let window = app.get_window("main")
        .ok_or(WindowError::NotFound)?;
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

/// Initialize event listeners for the main window
fn setup_window_events(window: &Window) -> WindowResult<()> {
    let window_clone = window.clone();
    window.listen("open_settings", move |_| {
        let window = window_clone.clone();
        tauri::async_runtime::spawn(async move {
            if let Err(e) = synapse_lib::commands::open_settings_window(window).await {
                error!("Failed to open settings window: {} ({:?})", e, e);
            }
        });
    });

    Ok(())
}

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();
    info!("Initializing Synapse application");

    let app = synapse_lib::create_app()
        .await
        .setup(|app| {
            info!("Setting up main application window");
            let window = app.get_window("main")
                .ok_or(WindowError::NotFound)?;
            
            // Initialize window systems
            window_management::setup_window_focus_handlers(&window)?;
            window_management::setup_window(&window)?;
            setup_window_events(&window)?;
            setup_global_shortcut(app)?;
            
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&window, true).expect("Failed to set window shadow");
            
            info!("Application initialization complete");
            Ok(())
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}
