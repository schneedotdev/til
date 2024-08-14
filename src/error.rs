use std::fmt::Display;

type Message = String;
type Directory = String;

#[derive(Debug, Default)]
pub(crate) enum Error {
    CannotBuildPath,
    CannotFindDir(Directory),
    CannotCreateDir(Directory),
    CannotProcessArgs,
    Custom(Message),
    #[default]
    Default,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    fn message(&self) -> Message {
        match self {
            Error::CannotBuildPath => "cannot construct path".to_owned(),
            Error::CannotFindDir(dir) => format!("cannot find {} directory", dir),
            Error::CannotCreateDir(dir) => format!("cannot create {} directory", dir),
            Error::CannotProcessArgs => "cannot process command-line arguments".to_owned(),
            Error::Custom(msg) => msg.to_owned(),
            Error::Default => "default error".to_owned(),
        }
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Custom(value)
    }
}
