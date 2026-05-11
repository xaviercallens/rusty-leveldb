```rust
#[inline]
pub fn internal_key_encoding_length(key: &ParsedInternalKey) -> usize {
    key.user_key.len() + std::mem::size_of::<u64>()
}
```