```rust
/// Packs a sequence number and value type into a single `u64`.
///
/// The upper 56 bits store `seq`, and the lower 8 bits store `t`.
///
/// # Panics
///
/// Panics if:
/// - `seq > k_max_sequence_number`
/// - `t > k_value_type_for_seek`
#[inline]
pub fn pack_sequence_and_type(seq: u64, t: ValueType) -> u64 {
    assert!(seq <= k_max_sequence_number && t <= k_value_type_for_seek);
    (seq << 8) | t
}
```