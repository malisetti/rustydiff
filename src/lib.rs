#![forbid(unsafe_code)]

pub mod diff;
pub mod error;
pub mod output;
pub mod parser;
pub mod record;

pub use diff::*;
pub use error::RustydiffError;
pub use output::Formatter;
pub use record::*;
