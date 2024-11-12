use crate::utils;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use windows::core::{w, PWSTR};
use windows::Win32::Foundation::{HWND, MAX_PATH};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER};
use windows::Win32::UI::Controls::Dialogs;
use windows::Win32::UI::Controls::Dialogs::{OFN_FILEMUSTEXIST, OFN_NOCHANGEDIR, OFN_PATHMUSTEXIST, OPENFILENAMEW};
use windows::Win32::UI::Shell::{FileOpenDialog, IFileDialog, FOS_PATHMUSTEXIST, FOS_PICKFOLDERS, SIGDN_FILESYSPATH};

pub fn open_directory_dialog(hwnd: HWND) -> Option<PathBuf>  {
    unsafe {
        let file_dialog: IFileDialog = CoCreateInstance(&FileOpenDialog, None, CLSCTX_INPROC_SERVER).unwrap();
        file_dialog.SetOptions(FOS_PICKFOLDERS | FOS_PATHMUSTEXIST)
            .expect("Error al establecer las opciones del diálogo.");

        match file_dialog.Show(hwnd) {
            Ok(_) => {
                let item = file_dialog.GetResult();
                if let Ok(item) = item {
                    let path = item.GetDisplayName(SIGDN_FILESYSPATH).unwrap();

                    Some(PathBuf::from(OsString::from_wide(path.as_wide())))
                }
                else {
                    println!("No se seleccionó ningún directorio.");
                    None
                }
            }
            Err(e) => {
                println!("No se seleccionó ningún directorio: {:?}", e);
                None
            }
        }
    }
}

pub fn open_file_dialog() -> Option<PathBuf> {
    let mut directory_path: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
    let mut settings = OPENFILENAMEW {
        lStructSize: size_of::<OPENFILENAMEW>() as u32,
        hwndOwner: HWND::default(),
        hInstance: Default::default(),
        lpstrFilter: w!(""),
        lpstrCustomFilter: PWSTR::null(),
        nMaxCustFilter: 0,
        nFilterIndex: 0,
        lpstrFile: PWSTR(directory_path.as_mut_ptr()),
        nMaxFile: MAX_PATH,
        nFileOffset: 0,
        nFileExtension: 0,
        lpstrFileTitle: PWSTR::null(),
        nMaxFileTitle: 0,
        lpstrInitialDir: w!(""),
        lpstrTitle: w!(""),
        Flags: OFN_NOCHANGEDIR | OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST,
        lpstrDefExt: w!(""),
        lCustData: Default::default(),
        lpfnHook: None,
        lpTemplateName: w!(""),
        pvReserved: std::ptr::null_mut(),
        dwReserved: 0,
        FlagsEx: Default::default(),
        ..Default::default()
    };

    let _ = unsafe { Dialogs::GetOpenFileNameW(&mut settings) };

    let directory_path = utils::trim_wide_null(&directory_path);
    if directory_path.is_empty() {
        None
    } else {
        Some(PathBuf::from(OsString::from_wide(&directory_path)))
    }
}