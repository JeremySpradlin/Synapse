// src-tauri/src/main.rs

use tauri::Manager;
use tauri::Window;
use tauri::GlobalShortcutManager;
use tauri::PhysicalPosition;
use tauri::Monitor;

// Command to get the current window position
#[tauri::command]
fn get_window_position(window: Window) -> (i32, i32) {
    if let Ok(position) = window.outer_position() {
        (position.x, position.y)
    } else {
        (0, 0)
    }
}

// Command to set the window position
#[tauri::command]
fn set_window_position(window: Window, x: i32, y: i32) {
    let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }));
}

// Function to center window horizontally on the primary monitor
fn center_window_horizontally(window: &Window, monitor: &Monitor) -> Result<(), Box<dyn std::error::Error>> {
    let screen_size = monitor.size();
    let window_size = window.outer_size()?;
    
    let x = (screen_size.width as i32 - window_size.width as i32) / 2;
    let y = 0; // Start at the top of the screen
    
    window.set_position(tauri::Position::Physical(PhysicalPosition { x, y }))?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            println!("Setting up application...");
            let window = app.get_window("main").unwrap();
            
            // Get the primary monitor
            if let Some(monitor) = window.primary_monitor()? {
                // Center the window horizontally at the top of the screen
                if let Err(e) = center_window_horizontally(&window, &monitor) {
                    println!("Error centering window: {}", e);
                }
            }
            
            // Register global shortcut (default: Command/Ctrl + Shift + Space)
            let mut shortcut_manager = app.global_shortcut_manager();
            let window_clone = window.clone();
            
            println!("Registering global shortcut...");
            shortcut_manager.register("CommandOrControl+Shift+Space", move || {
                println!("Shortcut triggered!");
                // Toggle window visibility
                if let Ok(is_visible) = window_clone.is_visible() {
                    println!("Current window visibility: {}", is_visible);
                    if is_visible {
                        let _ = window_clone.hide();
                        println!("Hiding window");
                    } else {
                        // When showing the window, first position it at the top
                        if let Ok(Some(monitor)) = window_clone.primary_monitor() {
                            let _ = center_window_horizontally(&window_clone, &monitor);
                        }
                        let _ = window_clone.show();
                        let _ = window_clone.set_focus();
                        println!("Showing window and setting focus");
                    }
                }
            })?;
            println!("Global shortcut registered successfully");

            // Set window to be transparent and always on top
            window.set_decorations(false)?;
            window.set_always_on_top(true)?;
            
            // Initially hide the window
            window.hide()?;
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
