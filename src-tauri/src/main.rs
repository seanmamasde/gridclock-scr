// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;

#[tauri::command]
fn toggle_fullscreen(w: Window) {
    let is_fullscreen = w.is_fullscreen().unwrap();
    w.set_fullscreen(!is_fullscreen).unwrap();
}

#[tauri::command]
fn toggle_exit(w: Window) {
    w.close().unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![toggle_fullscreen, toggle_exit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
