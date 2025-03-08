use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RespError {
    #[error("Invalid frame: {0}")]
    InvalidFrame(String),
    #[error("Invalid frame type: {0}")]
    InvalidFrameType(String),
    #[error("Invalid frame length")]
    InvalidFrameLength(isize),
    #[error("Frame is not complete")]
    NotComplete,
}
