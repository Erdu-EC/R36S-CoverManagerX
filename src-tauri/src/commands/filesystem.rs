use std::path::PathBuf;
use crate::modules::filesystem::get_removable_devices;
use crate::modules::filesystem::logical_device::LogicalDevice;

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