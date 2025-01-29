// src-tauri/src/main.rs

//! Synapse - A modern application launcher and command palette
//! 
//! This module contains the main application logic for the Synapse launcher,
//! handling window management, global shortcuts, and system integration.

use tauri::{
    GlobalShortcutManager, Manager, PhysicalPosition, Monitor, Window,
};

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

/// Sets up window focus event handlers
/// 
/// Configures automatic window hiding when focus is lost and other
/// focus-related behaviors.
fn setup_window_focus_handlers(window: &Window) -> WindowResult<()> {
    let win = window.clone();
    
    window.on_window_event(move |event| {
        match event {
            tauri::WindowEvent::Focused(focused) => {
                if !focused {
                    // Automatically hide window when focus is lost
                    if let Err(e) = win.hide() {
                        eprintln!("Failed to hide window: {}", e);
                    }
                }
            }
            _ => {}
        }
    });

    Ok(())
}

/// Initializes the application window
/// 
/// Sets up the window's initial state, position, and appearance.
fn setup_window(window: &Window) -> WindowResult<()> {
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

/// Sets up the global shortcut for toggling window visibility
/// 
/// Configures Command/Ctrl + Shift + Space as the global shortcut
/// for showing/hiding the window.
fn setup_global_shortcut(app: &tauri::App) -> WindowResult<()> {
    let window = app.get_window("main")
        .ok_or("Failed to get main window")?;
    let mut shortcut_manager = app.global_shortcut_manager();
    
    println!("Registering global shortcut...");
    shortcut_manager.register("CommandOrControl+Shift+Space", move || {
        println!("Shortcut triggered!");
        
        if let Ok(is_visible) = window.is_visible() {
            println!("Current window visibility: {}", is_visible);
            
            if is_visible {
                let _ = window.hide();
                println!("Hiding window");
            } else {
                // Reposition and show window
                if let Ok(Some(monitor)) = window.primary_monitor() {
                    let _ = center_window_horizontally(&window, &monitor);
                }
                let _ = window.show();
                let _ = window.set_focus();
                let _ = window.set_always_on_top(true);
                println!("Showing window and setting focus");
            }
        }
    })?;
    
    println!("Global shortcut registered successfully");
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("Setting up application...");

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
            let window = app.get_window("main")
                .ok_or("Failed to get main window")?;
            
            // Initialize window systems
            setup_window_focus_handlers(&window)?;
            setup_window(&window)?;
            setup_global_shortcut(app)?;
            
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&window, true).expect("Failed to set window shadow");
            
            println!("Window initialized and hidden");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
