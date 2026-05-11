```rust
/// Encodes a `u32` into 4 bytes in little-endian order.
///
/// # Panics
///
/// Panics if `dst` has fewer than 4 bytes.
#[inline]
pub fn encode_fixed32(dst: &mut [u8], value: u32) {
    dst[..4].copy_from_slice(&value.to_le_bytes());
}
```