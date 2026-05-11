```rust
use std::fmt;

/// A simple logger trait that accepts preformatted arguments.
pub trait Logger {
    /// Logs a formatted message.
    fn logv(&self, args: fmt::Arguments<'_>);
}

/// Logs a formatted message if a logger is provided.
pub fn log(info_log: Option<&dyn Logger>, args: fmt::Arguments<'_>) {
    info_log.map(|logger| logger.logv(args));
}

/// Convenience macro to call [`log`] with `format_args!` syntax.
#[macro_export]
macro_rules! log_msg {
    ($logger:expr, $($arg:tt)*) => {
        $crate::log($logger, format_args!($($arg)*))
    };
}
```