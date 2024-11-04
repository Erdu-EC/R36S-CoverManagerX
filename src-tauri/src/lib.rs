use crate::commands::filesystem::{
    filesystem_get_easyroms_device_path,
    filesystem_get_emulators,
};

mod commands;
mod modules;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })

        .invoke_handler(tauri::generate_handler![
            filesystem_get_easyroms_device_path,
            filesystem_get_emulators
        ])

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

