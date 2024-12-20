use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use tauri::ipc::InvokeError;
use tauri::{Window};
use crate::modules::windows;
use crate::modules::filesystem::{directory, get_removable_devices};
use crate::modules::filesystem::logical_device::LogicalDevice;
use crate::modules::r36s;
use crate::modules::r36s::{EmulatorData, RomData};

#[tauri::command]
pub fn filesystem_get_easyroms_device_path() -> Option<PathBuf> {
    let easy_roms_device = get_removable_devices()
        .iter()
        .map(|dp| LogicalDevice::from(dp.as_path()))
        .filter(|d| d.get_label().unwrap().eq("EASYROMS"))
        .collect::<Vec<LogicalDevice>>();

    match easy_roms_device.first() {
        Some(device) => Some(device.get_path().to_path_buf()),
        None => None,
    }
}

#[tauri::command]
pub fn filesystem_get_emulators(dir: String) -> Result<Vec<EmulatorData>, InvokeError> {
    let directories = directory::get_dir_entries(&dir);
    let directories = match directories {
        Ok(d) => d.iter()
            .filter(|d| d.is_dir() && d.file_name().is_some())
            .map(|d| d.file_name().unwrap().to_os_string())
            .collect::<Vec<_>>(),
        Err(e) => {
            return Err(InvokeError::from_anyhow(anyhow::Error::new(e)));
        }
    };

    let emulators = r36s::get_emulator_data()
        .into_iter()
        .filter(|e| directories.contains(&OsString::from(e.name)))
        .collect::<Vec<_>>();

    Ok(emulators)
}

#[tauri::command]
pub async  fn filesystem_get_roms(dir: String, extensions: Vec<&str>) -> Result<Vec<RomData>, InvokeError> {
    let files = directory::get_files(&dir, if extensions.len() > 0 {
        Some(extensions.iter().map(|e| OsString::from(e)).collect::<_>())
    } else {
        None
    });

    if let Err(e) = files {
        return Err(InvokeError::from_anyhow(anyhow::Error::new(e)));
    }

    let files = files.unwrap();
    let mut roms = Vec::new();

    for f in files {
        let name = f.file_stem().unwrap();
        let extension = f.extension().unwrap();
        let length = match f.metadata() {
            Ok(m) => m.len(),
            Err(_) => 0,
        };

        roms.push(RomData {
            name: name.encode_wide().collect(),
            extension: extension.encode_wide().collect(),
            path: f,
            length,
        });
    }

    Ok(roms)
}

#[tauri::command]
pub async fn filesystem_get_game_list(dir: String) -> Result<Vec<r36s::game_list::GameListEntry>, InvokeError> {
    r36s::game_list::get_game_list(&PathBuf::from(dir))
        .map_err(|e| InvokeError::from_anyhow(e))
}

#[tauri::command]
pub fn filesystem_open_directory_dialog(window: Window) -> Option<PathBuf> {
    let hwnd = window.hwnd().expect("Error al obtener el identificador de la ventana.");
    windows::dialogs::open_directory_dialog(hwnd)
}