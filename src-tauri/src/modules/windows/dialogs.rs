use windows::core::{w, PWSTR};
use windows::Win32::Foundation::{HWND, MAX_PATH};
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER};
use windows::Win32::UI::Controls::Dialogs;
use windows::Win32::UI::Controls::Dialogs::{OFN_FILEMUSTEXIST, OFN_NOCHANGEDIR, OFN_PATHMUSTEXIST, OPENFILENAMEW};
use windows::Win32::UI::Shell::{FileOpenDialog, IFileDialog, FOS_PICKFOLDERS, SIGDN_FILESYSPATH};

pub fn open_directory_dialog(hwnd: HWND) {
    let file_dialog: IFileDialog = unsafe { CoCreateInstance(&FileOpenDialog, None, CLSCTX_INPROC_SERVER).unwrap() };
    // Configura el diálogo para seleccionar carpetas

    unsafe {
        file_dialog.SetOptions(FOS_PICKFOLDERS).expect("TODO: panic message");

        // Muestra el diálogo al usuario
        if file_dialog.Show(hwnd).is_ok() {
            // Obtiene el resultado de la carpeta seleccionada
            let item = file_dialog.GetResult();

            if let Ok(item) = item {
                // Obtiene la ruta de la carpeta seleccionada
                let path = item.GetDisplayName(SIGDN_FILESYSPATH).unwrap();
                println!("Directorio seleccionado: {:?}", path.to_string());
            }
        } else {
            println!("No se seleccionó ningún directorio.");
        }
    }

    ()
}

pub fn open_file_dialog() {
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
}