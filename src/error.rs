use std::fmt;

pub enum Error {
    Import(ImportError),
}

pub enum ImportError {
    FileNotFound,
    InvalidExtention,
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FileNotFound => write!(f, "file not found"),
            Self::InvalidExtention => write!(f, "invalid extention"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Import(err) => write!(f, "Import failed: {}", err),
        }
    }
}