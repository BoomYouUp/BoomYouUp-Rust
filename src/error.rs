pub type FinalResult<T = ()> = Result<T, UnexpectedError>;
pub type NormalResult<T = ()> = Result<T, NormalError>;
pub type DetailedResult<T = ()> = Result<NormalResult<T>, UnexpectedError>;

#[derive(Debug)]
pub enum UnexpectedError {
    Io(std::io::Error),
    SerializationAndDeserialization(serde_yaml::Error),
}

impl std::fmt::Display for UnexpectedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnexpectedError::Io(e) => write!(f, "I/O 错误：{}", e),
            UnexpectedError::SerializationAndDeserialization(e) => {
                write!(f, "序列化和反序列化错误：{}", e)
            }
        }
    }
}

impl std::error::Error for UnexpectedError {}

impl From<std::io::Error> for UnexpectedError {
    fn from(e: std::io::Error) -> Self {
        UnexpectedError::Io(e)
    }
}

impl From<serde_yaml::Error> for UnexpectedError {
    fn from(e: serde_yaml::Error) -> Self {
        UnexpectedError::SerializationAndDeserialization(e)
    }
}

#[derive(Debug)]
pub enum NormalError {
    Input,
    NumberFormat,
    Cancelled,
    Execution(opener::OpenError),
    Play(soloud::SoloudError),
    Notify(notify_rust::error::Error),
}

impl std::fmt::Display for NormalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NormalError::Input => write!(f, "输入错误"),
            NormalError::NumberFormat => write!(f, "数字格式错误"),
            NormalError::Cancelled => write!(f, "操作已取消"),
            NormalError::Execution(e) => write!(f, "命令执行错误：{}", e),
            NormalError::Play(e) => write!(f, "音频播放错误：{}", e),
            NormalError::Notify(e) => write!(f, "通知发送错误：{}", e),
        }
    }
}

impl std::error::Error for NormalError {}

impl From<opener::OpenError> for NormalError {
    fn from(e: opener::OpenError) -> Self {
        NormalError::Execution(e)
    }
}

impl From<soloud::SoloudError> for NormalError {
    fn from(e: soloud::SoloudError) -> Self {
        NormalError::Play(e)
    }
}

impl From<notify_rust::error::Error> for NormalError {
    fn from(e: notify_rust::error::Error) -> Self {
        NormalError::Notify(e)
    }
}

pub struct PrintingArgs {
    pub ok_message: Option<String>,
    pub err_message: String,
}

impl PrintingArgs {
    pub fn unexpected() -> Self {
        PrintingArgs {
            ok_message: None,
            err_message: "错误".to_string(),
        }
    }

    pub fn normal() -> Self {
        PrintingArgs {
            ok_message: None,
            err_message: "遇到了问题".to_string(),
        }
    }

    pub fn customized(err_message: &str) -> Self {
        PrintingArgs {
            ok_message: None,
            err_message: err_message.to_string(),
        }
    }

    pub fn ok_message(mut self, message: &str) -> Self {
        self.ok_message = Some(message.to_string());
        self
    }

    pub fn err_message(mut self, message: &str) -> Self {
        self.err_message = message.to_string();
        self
    }
}

pub trait ResultPrinting {
    fn result_println_then(&self, args: PrintingArgs) -> &Self;

    fn result_println(&self, args: PrintingArgs) {
        self.result_println_then(args);
    }
}

impl<T, E: std::error::Error> ResultPrinting for Result<T, E> {
    fn result_println_then(&self, args: PrintingArgs) -> &Self {
        match self {
            Ok(_) => {
                if let Some(message) = args.ok_message {
                    println!("{message}");
                }
            }
            Err(e) => {
                eprintln!("{}：{e}", args.err_message);
            }
        }

        self
    }
}

impl<E: std::error::Error + _Error> ResultPrinting for E {
    fn result_println_then(&self, args: PrintingArgs) -> &Self {
        eprintln!("{}：{self}", args.err_message);

        self
    }
}

pub trait _Error {}
impl _Error for NormalError {}
impl _Error for UnexpectedError {}
