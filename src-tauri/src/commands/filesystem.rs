use std::ffi::OsString;
use std::path::PathBuf;
use tauri::ipc::InvokeError;
use crate::modules::filesystem::{directory, get_removable_devices};
use crate::modules::filesystem::logical_device::LogicalDevice;
use crate::modules::r36s;
use crate::modules::r36s::EmulatorData;

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
pub fn filesystem_get_roms(dir: String, extensions: Vec<&str>) -> Result<Vec<PathBuf>, InvokeError> {
    let files = directory::get_files(&dir, if extensions.len() > 0 {
        Some(extensions.iter().map(|e| OsString::from(e)).collect::<_>())
    } else {
        None
    });

    match files {
        Ok(f) => Ok(f),
        Err(e) => Err(InvokeError::from_anyhow(anyhow::Error::new(e))),
    }
}