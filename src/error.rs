use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RustydiffError {
    #[error("parse error: {0}")]
    Parse(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("argument error: {0}")]
    Argument(String),
}
