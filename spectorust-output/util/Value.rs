```rust
/// Computes the checksum value for `data` by extending from an initial value of `0`.
#[inline]
pub fn value(data: &[u8]) -> u32 {
    extend(0, data)
}
```