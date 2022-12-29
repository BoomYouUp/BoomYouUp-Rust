#[macro_export]
macro_rules! pcstr {
    ($s:expr) => {
        windows::core::PCSTR::from_raw(($s + "\0").as_ptr())
    };
}
