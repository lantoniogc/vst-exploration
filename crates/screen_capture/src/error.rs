use image::ImageError;
use thiserror::Error;
use x11rb::errors::{ConnectError, ConnectionError, ReplyError};


#[derive(Error, Debug)]
pub enum CaptureError {
    #[error("XCBError: {:?}", .0)]
    XCBError(String),
    #[error("ImageError: {:?}", .0)]
    ImageError(String),
}

impl From<ConnectError> for CaptureError {
    fn from(err: ConnectError) -> Self {
        CaptureError::XCBError(err.to_string())
    }
}

impl From<ConnectionError> for CaptureError {
    fn from(err: ConnectionError) -> Self {
        CaptureError::XCBError(err.to_string())
    }
}

impl From<ReplyError> for CaptureError {
    fn from(err: ReplyError) -> Self {
        CaptureError::XCBError(err.to_string())
    }
}

impl From<ImageError> for CaptureError {
    fn from(err: ImageError) -> Self {
        CaptureError::ImageError(err.to_string())
    }
}
