use std::{fmt::Display, path::PathBuf};

type Message = String;
type Directory = String;

#[derive(Debug, Default)]
pub(crate) enum Error {
    CannotBuildPath,
    CannotFindDir(Directory),
    CannotCreateDir(Directory),
    CannotProcessArgs,
    CannotOpenOrCreatePath(PathBuf),
    CannotWriteToFile(PathBuf),
    CannotReadFile(PathBuf),
    Custom(Message),
    #[default]
    Default,
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CannotBuildPath => f.write_str("cannot construct path"),
            Error::CannotFindDir(dir) => f.write_fmt(format_args!("cannot find {} directory", dir)),
            Error::CannotCreateDir(dir) => {
                f.write_fmt(format_args!("cannot create {} directory", dir))
            }
            Error::CannotProcessArgs => f.write_str("cannot process command-line arguments"),
            Error::CannotOpenOrCreatePath(path) => {
                f.write_fmt(format_args!("cannot open or create {}", path.display()))
            }
            Error::CannotWriteToFile(file) => {
                f.write_fmt(format_args!("cannot write to {}", file.display()))
            }
            Error::CannotReadFile(file) => {
                f.write_fmt(format_args!("cannot read file {}", file.display()))
            }
            Error::Custom(msg) => f.write_str(msg),
            Error::Default => f.write_str("something wrong happened"),
        }
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::Custom(value.to_string())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Custom(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;

    #[test]
    fn error_variants_return_proper_messages() {
        let pairs: &[(Error, &str)] = &[
            (Error::CannotBuildPath, "cannot construct path"),
            (
                Error::CannotFindDir("parent".to_string()),
                "cannot find parent directory",
            ),
            (
                Error::CannotCreateDir("./til/notes".to_string()),
                "cannot create ./til/notes directory",
            ),
            (
                Error::CannotProcessArgs,
                "cannot process command-line arguments",
            ),
            (
                Error::CannotOpenOrCreatePath("src/test".into()),
                "cannot open or create src/test",
            ),
            (
                Error::CannotWriteToFile("src/test".into()),
                "cannot write to src/test",
            ),
            (
                Error::CannotReadFile("src/test".into()),
                "cannot read file src/test",
            ),
            ("custom message".into(), "custom message"),
            (Error::default(), "something wrong happened"),
        ];

        pairs
            .iter()
            .for_each(|(err, msg)| assert_eq!(format!("{err}"), msg.to_string()));
    }
}
