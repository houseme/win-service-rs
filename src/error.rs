use std::fmt;
use windows::Win32::Foundation::WIN32_ERROR;

#[derive(Debug)]
pub enum Error {
    Windows(WIN32_ERROR),
    Io(std::io::Error),
    InvalidPath(String),
    AccessDenied,
    ServiceNotFound,
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Windows(e) => write!(f, "Windows error: {}", e.0),
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::InvalidPath(p) => write!(f, "Invalid path: {}", p),
            Error::AccessDenied => write!(f, "Access denied"),
            Error::ServiceNotFound => write!(f, "Service not found"),
            Error::Other(s) => write!(f, "Error: {}", s),
        }
    }
}

impl std::error::Error for Error {}

impl From<WIN32_ERROR> for Error {
    fn from(err: WIN32_ERROR) -> Self {
        match err.0 {
            5 => Error::AccessDenied,
            1060 => Error::ServiceNotFound,
            _ => Error::Windows(err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
