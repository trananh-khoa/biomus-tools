// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// https://github.com/tauri-apps/window-vibrancy/blob/dev/examples/tauri/src-tauri/src/main.rs
use tauri::Manager;

#[allow(unused_imports)]
use window_vibrancy::{apply_acrylic, apply_vibrancy, NSVisualEffectMaterial};

// Custom modules
mod common;
mod file_converter;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            #[allow(unused_code)]
            #[cfg(target_os = "windows")]
            apply_acrylic(&window, Some((0, 0, 0, 10)))
                .expect("Unsupported platform! 'apply_acrylic' is only supported on Windows");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![file_converter::convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
