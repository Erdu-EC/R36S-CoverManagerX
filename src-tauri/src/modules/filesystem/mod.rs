pub mod logical_device;
pub mod directory;

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use windows::Win32::Storage::FileSystem;
use crate::modules::enums::DriveTypes;
use crate::utils::pcwstr;

fn get_logical_devices() -> Vec<PathBuf> {
    let letters = &mut [0; 130];
    unsafe {
        let result_length = FileSystem::GetLogicalDriveStringsW(Option::from(letters.as_mut()));
        if result_length > letters.len() as u32 {
            panic!("Error getting logical devices: {}", result_length);
        }
    };

    letters
        .split(|c| *c == 0x00)
        .filter(|s| !s.is_empty())
        .map(|c| PathBuf::from(OsString::from_wide(c)))
        .collect::<Vec<_>>()
}

pub fn get_removable_devices() -> Vec<PathBuf> {
    get_logical_devices().into_iter()
        .filter(|d| unsafe { FileSystem::GetDriveTypeW(pcwstr::from_path(d)) } == DriveTypes::DriveRemovable as u32)
        .collect::<Vec<_>>()
}
