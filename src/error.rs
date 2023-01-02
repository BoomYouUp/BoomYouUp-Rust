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
            UnexpectedError::Io(e) => write!(f, "I/O 错误: {}", e),
            UnexpectedError::SerializationAndDeserialization(e) => {
                write!(f, "序列化和反序列化错误: {}", e)
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
}

impl std::fmt::Display for NormalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NormalError::Input => write!(f, "输入错误"),
            NormalError::NumberFormat => write!(f, "数字格式错误"),
            NormalError::Cancelled => write!(f, "操作已取消"),
        }
    }
}

impl std::error::Error for NormalError {}

pub struct PrintingArgs {
    pub message: Option<String>,
}

impl PrintingArgs {
    pub fn new() -> Self {
        PrintingArgs { message: None }
    }

    pub fn normal() -> Self {
        PrintingArgs {
            message: Some("遇到了问题".to_string()),
        }
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
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
                if let Some(message) = args.message {
                    println!("{message}");
                }
            }
            Err(e) => {
                eprintln!(
                    "{}：{e}",
                    args.message.unwrap_or_else(|| "错误".to_string())
                );
            }
        }

        self
    }
}

impl<E: std::error::Error + _Error> ResultPrinting for E {
    fn result_println_then(&self, args: PrintingArgs) -> &Self {
        eprintln!(
            "{}：{self}",
            args.message.unwrap_or_else(|| "错误".to_string())
        );

        self
    }
}

pub trait _Error {}
impl _Error for NormalError {}
impl _Error for UnexpectedError {}
