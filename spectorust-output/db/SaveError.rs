```rust
/// Stores an error message into `errptr` if `status` is an error.
///
/// Returns `true` when an error was saved, `false` when `status` is OK.
///
/// This is an idiomatic Rust replacement for a C-style `char**` out-parameter:
/// - `None` means no stored error.
/// - `Some(String)` holds the latest error message.
fn save_error(errptr: &mut Option<String>, status: &Status) -> bool {
    (!status.ok()).then(|| *errptr = Some(status.to_string())).is_some()
}
```