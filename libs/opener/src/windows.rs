use crate::OpenError;
use std::ffi::OsStr;
use std::io;
use std::os::windows::ffi::OsStrExt;
use windows::core::PCWSTR;
use windows::w;
use windows::Win32::Foundation::{GetLastError, HWND};
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

pub(crate) fn open(path: &OsStr, parameters: &OsStr) -> Result<(), OpenError> {
    let path = convert_str(path).map_err(OpenError::Io)?;
    let parameters = convert_str(parameters).map_err(OpenError::Io)?;
    unsafe {
        ShellExecuteW(
            HWND::default(),
            w!("open"),
            PCWSTR::from_raw(path.as_ptr()),
            PCWSTR::from_raw(parameters.as_ptr()),
            PCWSTR::null(),
            SW_SHOWNORMAL,
        );

        let result = GetLastError().to_hresult();
        if result.is_err() {
            Err(OpenError::Io(io::Error::last_os_error()))
        } else {
            Ok(())
        }
    }
}

fn convert_str(s: &OsStr) -> io::Result<Vec<u16>> {
    let mut maybe_result: Vec<u16> = s.encode_wide().collect();
    if maybe_result.iter().any(|&u| u == 0) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "path contains NUL byte(s)",
        ));
    }

    maybe_result.push(0);
    Ok(maybe_result)
}
