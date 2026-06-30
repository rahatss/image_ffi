use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Image(image::ImageError),
    Plugin(libloading::Error),
    InvalidInput(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {e}"),
            AppError::Image(e) => write!(f, "Image error: {e}"),
            AppError::Plugin(e) => write!(f, "Plugin error: {e}"),
            AppError::InvalidInput(s) => write!(f, "Invalid input: {s}"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            AppError::Image(err) => Some(err),
            AppError::Plugin(err) => Some(err),
            AppError::InvalidInput(_) => None,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<image::ImageError> for AppError {
    fn from(e: image::ImageError) -> Self {
        AppError::Image(e)
    }
}

impl From<libloading::Error> for AppError {
    fn from(e: libloading::Error) -> Self {
        AppError::Plugin(e)
    }
}
