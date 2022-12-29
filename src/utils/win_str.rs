#[macro_export]
macro_rules! pcstr {
    ($s:expr) => {
        windows::core::PCSTR::from_raw((String::from($s) + "\0").as_ptr())
    };
}
