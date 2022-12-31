use crate::OpenError;
use std::ffi::OsStr;
use std::process::{Command, Stdio};

pub(crate) fn open(path: &OsStr, parameters: &OsStr) -> Result<(), OpenError> {
    if parameters.is_empty() {
        let mut open = Command::new("open")
            .arg(path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(OpenError::Io)?;

        crate::wait_child(&mut open, "open")
    } else {
        let mut open = Command::new("open")
            .arg(path)
            .arg("--args")
            .arg(parameters)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(OpenError::Io)?;

        crate::wait_child(&mut open, "open")
    }
}
