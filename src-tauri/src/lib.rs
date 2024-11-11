use crate::commands::filesystem::{filesystem_get_easyroms_device_path, filesystem_get_emulators, filesystem_get_game_list, filesystem_get_roms, filesystem_open_directory_dialog};

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
            filesystem_get_emulators,
            filesystem_get_roms,
            filesystem_get_game_list,
            filesystem_open_directory_dialog
        ])

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

