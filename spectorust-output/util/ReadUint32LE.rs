```rust
/// Reads a little-endian `u32` from the first 4 bytes of `buffer`.
///
/// Returns `None` if `buffer` has fewer than 4 bytes.
#[inline]
pub fn read_uint32_le(buffer: &[u8]) -> Option<u32> {
    Some(u32::from_le_bytes(buffer.get(..4)?.try_into().ok()?))
}
```