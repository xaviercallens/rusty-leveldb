```rust
use std::io;

/// Represents an operation status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    /// Resource was not found.
    NotFound { context: String, message: String },
    /// Generic I/O error.
    IOError { context: String, message: String },
}

impl Status {
    /// Creates a `NotFound` status.
    pub fn not_found(context: impl Into<String>, message: impl Into<String>) -> Self {
        Self::NotFound {
            context: context.into(),
            message: message.into(),
        }
    }

    /// Creates an `IOError` status.
    pub fn io_error(context: impl Into<String>, message: impl Into<String>) -> Self {
        Self::IOError {
            context: context.into(),
            message: message.into(),
        }
    }
}

/// Converts a POSIX errno and context into a `Status`.
pub fn posix_error(context: &str, error_number: i32) -> Status {
    let message = io::Error::from_raw_os_error(error_number).to_string();
    match error_number {
        libc::ENOENT => Status::not_found(context, message),
        _ => Status::io_error(context, message),
    }
}
```