#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{WebviewUrl, WebviewWindowBuilder};

fn main() {
    println!("Starting Zalo of Linux (Tauri V2 + Vite)...");
    
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
