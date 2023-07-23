//! A dedicated error for all possible errors in DNS queries: I/O, DNS packet unconsistencies, etc
use std::array::TryFromSliceError;
use std::io;
use std::net::AddrParseError;
use std::str;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FromUtf8(std::string::FromUtf8Error),
    Utf8(str::Utf8Error),
    AddrParseError(AddrParseError),
    FromSlice(TryFromSliceError),
    #[cfg(target_family = "windows")]
    Windows(u32),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::FromUtf8(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self {
        Error::Utf8(err)
    }
}

impl From<AddrParseError> for Error {
    fn from(err: AddrParseError) -> Self {
        Error::AddrParseError(err)
    }
}

impl From<TryFromSliceError> for Error {
    fn from(err: TryFromSliceError) -> Self {
        Error::FromSlice(err)
    }
}

#[cfg(target_family = "windows")]
impl From<u32> for Error {
    fn from(err: u32) -> Self {
        Error::Windows(err)
    }
}
