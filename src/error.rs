use std::fmt;

pub enum Error {
    Import(ImportError),
    Cancel,
    Unknown,
}

pub enum ImportError {
    FileNotFound,
    InvalidExtention,
    FileCorrupted,
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FileNotFound => write!(f, "file not found"),
            Self::InvalidExtention => write!(f, "invalid extention"),
            Self::FileCorrupted => write!(f, "file corrupted"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Import(err) => write!(f, "Import failed: {}", err),
            Self::Cancel => write!(f, "Canceled"),
            Self::Unknown => write!(f, "Unknown error"),
        }
    }
}