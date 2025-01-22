// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    Manager,
};
use tauri_plugin_global_shortcut::GlobalShortcut;

fn main() {
    tauri::Builder::default()
        .plugin(GlobalShortcut::default())
        .setup(|app| {
            let handle = app.handle();
            let handle_clone = handle.clone();
            
            // Create quit menu item
            let quit = MenuItemBuilder::with_id("quit", "Quit")
                .build(&handle_clone)?;
            
            // Build menu
            let menu = MenuBuilder::new(&handle_clone)
                .item(&quit)
                .build()?;

            // Create tray icon with menu
            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .build(handle.clone())?;

            // Register global shortcut
            GlobalShortcut::register(handle, "Shift+T", || {
                println!("Shift+T pressed!");
                // TODO: Add your shortcut handler logic here
            })?;

            Ok(())
        })
        .on_menu_event(|app, event| {
            if event.id().0 == "quit" {
                std::process::exit(0);
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
