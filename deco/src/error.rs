use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    IoErr(std::io::Error),
    TcGetAttrErr,
    TcSetAttrErr,
    ReadBytesErr,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoErr(err) => write!(f, "{}", err),
            Self::TcGetAttrErr => write!(f, "tcgetattr"),
            Self::TcSetAttrErr => write!(f, "tcsetattr"),
            Self::ReadBytesErr => write!(f, "error occured while reading bytes"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoErr(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
