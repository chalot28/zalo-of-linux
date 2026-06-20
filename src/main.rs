#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{WebviewUrl, WebviewWindowBuilder};

fn main() {
    println!("Starting Zalo of Linux (Tauri V2)...");
    
    tauri::Builder::default()
        .setup(|app| {
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External("https://chat.zalo.me/".parse().unwrap()))
                .title("Zalo of Linux")
                .inner_size(1280.0, 800.0)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
