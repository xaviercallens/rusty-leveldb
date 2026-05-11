```rust
/// Initializes CRC32C values for each record type from `0..=k_max_record_type`.
///
/// Each entry `type_crc[i]` is the CRC32C of a single byte containing `i`.
pub fn init_type_crc(type_crc: &mut [u32], k_max_record_type: usize) {
    assert!(
        type_crc.len() > k_max_record_type,
        "type_crc must have at least k_max_record_type + 1 elements"
    );

    type_crc
        .iter_mut()
        .take(k_max_record_type + 1)
        .enumerate()
        .for_each(|(i, slot)| *slot = crc32c::crc32c(&[i as u8]));
}
```