```rust
use std::io::{self, Write};

/// Iterates through all entries and dumps parsed internal keys to stderr.
///
/// For each key:
/// - If parsing fails, prints: `Corrupt '<escaped-key>'`
/// - If parsing succeeds, prints: `@ '<debug-string>'`
pub fn dump_internal_iter<I>(iter: &mut I)
where
    I: IteratorLike,
{
    let mut stderr = io::stderr().lock();
    iter.seek_to_first();
    while iter.valid() {
        let key = iter.key();
        let line = match parse_internal_key(key) {
            Some(k) => format!("@ '{}'", k.debug_string()),
            None => format!("Corrupt '{}'", escape_string(key)),
        };
        let _ = writeln!(stderr, "{line}");
        iter.next();
    }
}

/// Minimal iterator interface matching the original C++ usage pattern.
pub trait IteratorLike {
    fn seek_to_first(&mut self);
    fn valid(&self) -> bool;
    fn next(&mut self);
    fn key(&self) -> &[u8];
}

/// Parsed representation of an internal key.
pub struct ParsedInternalKey {
    debug: String,
}

impl ParsedInternalKey {
    pub fn debug_string(&self) -> &str {
        &self.debug
    }
}

/// Parses an internal key. Returns `None` if the key is corrupt.
pub fn parse_internal_key(_key: &[u8]) -> Option<ParsedInternalKey> {
    None
}

/// Escapes a raw key for human-readable output.
pub fn escape_string(key: &[u8]) -> String {
    key.iter()
        .flat_map(|&b| std::ascii::escape_default(b))
        .map(char::from)
        .collect()
}
```