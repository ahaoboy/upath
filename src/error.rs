//! Error type for the `upath` crate.

use std::fmt;

/// Errors that can be produced while resolving a unique path.
#[derive(Debug)]
pub enum Error {
    /// An underlying I/O error, for example while reading a directory.
    Io(std::io::Error),
    /// The numeric suffix counter overflowed `u64`. Practically unreachable,
    /// but handled so the search can never loop forever.
    Overflow,
    /// The supplied path has no file-name component (for example a filesystem
    /// root), so it cannot be uniquified.
    MissingFileName,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "i/o error: {e}"),
            Error::Overflow => write!(f, "numeric suffix counter overflowed"),
            Error::MissingFileName => write!(f, "path has no file name to uniquify"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Overflow | Error::MissingFileName => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

/// Convenient alias used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;
