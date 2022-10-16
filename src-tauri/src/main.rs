#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Plugin {
    source: String,
}

#[tauri::command]
fn all_plugins() -> Result<Vec<Plugin>, String> {
    let mut plugins = vec![];
    for entry in std::fs::read_dir(std::env::current_dir().unwrap().join("plugins"))
        .map_err(|_| "plugins dir not found")?
        .flatten()
    {
        if entry.path().is_file() {
            let source = std::fs::read_to_string(entry.path())
                .map_err(|_| format!("failed to read file: {}", entry.path().display()))?;
            plugins.push(Plugin { source });
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
