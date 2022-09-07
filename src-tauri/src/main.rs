#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

mod plugin;

#[derive(Debug, Serialize, Deserialize)]
struct Plugin {
    wasm_binary: Vec<u8>,
}

#[tauri::command]
fn all_plugins() -> Result<Vec<Plugin>, String> {
    let mut plugins = vec![];
    for entry in std::fs::read_dir(std::env::current_dir().unwrap().join("plugins"))
        .map_err(|_| "plugins dir not found")?
        .flatten()
    {
        if entry.path().is_file() {
            let mut file = File::open(entry.path())
                .map_err(|_| format!("failed to open file: {}", entry.path().display()))?;
            let mut wasm_binary = Vec::new();
            file.read_to_end(&mut wasm_binary)
                .map_err(|_| format!("failed to read binary: {}", entry.path().display()))?;
            plugins.push(Plugin { wasm_binary });
        }
    }
    Ok(plugins)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                use tauri::Manager;
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![all_plugins])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
