```rust
/// Decodes an entry header from `input`, returning the remaining slice on success.
///
/// The header encodes three `u32` values in order:
/// - `shared`
/// - `non_shared`
/// - `value_length`
///
/// Fast path: if the first three bytes are all `< 128`, each value is a single byte.
/// Otherwise, each value is decoded as a varint32.
///
/// Returns `None` if decoding fails or if there are not enough remaining bytes
/// for `non_shared + value_length`.
pub fn decode_entry(input: &[u8]) -> Option<(&[u8], u32, u32, u32)> {
    let (rest, shared, non_shared, value_length) = match input {
        [b0, b1, b2, rest @ ..] if (*b0 | *b1 | *b2) < 128 => {
            (rest, *b0 as u32, *b1 as u32, *b2 as u32)
        }
        _ => {
            let (r1, shared) = get_varint32_ptr(input)?;
            let (r2, non_shared) = get_varint32_ptr(r1)?;
            let (r3, value_length) = get_varint32_ptr(r2)?;
            (r3, shared, non_shared, value_length)
        }
    };

    let needed = non_shared.checked_add(value_length)? as usize;
    (rest.len() >= needed).then_some((rest, shared, non_shared, value_length))
}

/// Decodes a little-endian base-128 varint32 from the start of `input`.
///
/// Returns `(remaining_input, value)` on success, or `None` if the varint is
/// malformed or truncated.
fn get_varint32_ptr(input: &[u8]) -> Option<(&[u8], u32)> {
    let mut result = 0u32;

    for (i, &byte) in input.iter().take(5).enumerate() {
        result |= ((byte & 0x7f) as u32) << (i as u32 * 7);
        if byte < 0x80 {
            return Some((&input[i + 1..], result));
        }
    }

    None
}
```