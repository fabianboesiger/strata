use std::{
    error,
    fmt
};
use image::ImageError;

#[derive(Clone, Debug)]
pub struct Error {
    message: String
}

impl Error {
    fn new(message: String) -> Error {
        Error {
            message
        }
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<ImageError> for Error {
    fn from(error: ImageError) -> Error {
        Error::new(format!("{}", error))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
