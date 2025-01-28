// src-tauri/src/main.rs

//! Synapse - A modern application launcher and command palette
//! 
//! This module contains the main application logic for the Synapse launcher,
//! handling window management, global shortcuts, and system integration.

use tauri::{
    Manager, Window, GlobalShortcutManager,
    PhysicalPosition, Monitor, LogicalPosition,
};

/// Error type for window management operations
type WindowResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Gets the current window position
/// 
/// Returns a tuple of (x, y) coordinates representing the window's position
/// on the screen. Returns (0, 0) if the position cannot be determined.
#[tauri::command]
fn get_window_position(window: Window) -> (i32, i32) {
    window.outer_position()
        .map(|pos| (pos.x, pos.y))
        .unwrap_or((0, 0))
}

/// Sets the window position to the specified coordinates
/// 
/// # Arguments
/// * `window` - The window to reposition
/// * `x` - The x coordinate
/// * `y` - The y coordinate
#[tauri::command]
fn set_window_position(window: Window, x: i32, y: i32) {
    let _ = window.set_position(tauri::Position::Physical(
        tauri::PhysicalPosition { x, y }
    ));
}

/// Centers the window horizontally at the top of the specified monitor
/// 
/// # Arguments
/// * `window` - The window to center
/// * `monitor` - The monitor to center the window on
fn center_window_horizontally(window: &Window, monitor: &Monitor) -> WindowResult<()> {
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

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            println!("Setting up application...");
            
            let window = app.get_window("main")
                .ok_or("Failed to get main window")?;
            
            // Initialize window systems
            setup_window_focus_handlers(&window)?;
            setup_window(&window)?;
            setup_global_shortcut(app)?;
            
            println!("Window initialized and hidden");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_window_position,
            set_window_position
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
