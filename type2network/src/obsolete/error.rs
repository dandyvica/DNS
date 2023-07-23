use std::fmt;
use std::io;
use std::num::TryFromIntError;

//#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    TryFromInt(TryFromIntError),
    Custom(String)
}

// All convertion for internal errors for DNSError
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<TryFromIntError> for Error {
    fn from(err: TryFromIntError) -> Self {
        Error::TryFromInt(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Custom(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::TryFromInt(ref err) => err.fmt(f),
            Error::Custom(ref err) => err.fmt(f),

        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::TryFromInt(ref err) => err.fmt(f),
            Error::Custom(ref err) => err.fmt(f),
        }
    }
}
