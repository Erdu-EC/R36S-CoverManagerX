use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use windows::core::Error;
use windows::Win32::Foundation::MAX_PATH;
use windows::Win32::Storage::FileSystem::GetVolumeInformationW;
use crate::utils;
use crate::utils::pcwstr;

pub struct LogicalDevice {
    path: PathBuf,
}

impl From<&Path> for LogicalDevice {
    fn from(path: &Path) -> Self {
        Self { path: path.to_path_buf() }
    }
}

impl LogicalDevice {
    pub fn get_path(&self) -> &Path {
        &self.path
    }
    
    pub fn get_label(&self) -> Result<OsString, Error> {
        let volume_name = &mut [0u16; MAX_PATH as usize];
        let file_system_name = &mut [0u16; MAX_PATH as usize];

        let mut volume_serial_number = 0u32;
        let mut max_component_length = 0u32;
        let mut file_system_flags = 0u32;

        let result = unsafe {
            GetVolumeInformationW(
                pcwstr::from_path(self.path.as_path()),
                Some(volume_name.as_mut()),
                Some(&mut volume_serial_number),
                Some(&mut max_component_length),
                Some(&mut file_system_flags),
                Some(file_system_name.as_mut()),
            )
        };

        match result {
            Ok(_) => Ok(OsString::from_wide(utils::trim_wide_null(volume_name))),
            Err(e) => Err(e),
        }
    }
}