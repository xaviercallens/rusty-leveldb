```rust
/// A lightweight byte slice view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Slice<'a> {
    data: &'a [u8],
}

impl<'a> Slice<'a> {
    /// Creates a new `Slice` from a byte slice.
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    /// Returns the underlying bytes.
    pub fn as_bytes(&self) -> &'a [u8] {
        self.data
    }
}

/// Decodes a varint32 from the beginning of `input`.
/// Returns `(value, bytes_consumed)` on success.
fn get_varint32(input: &[u8]) -> Option<(u32, usize)> {
    input
        .iter()
        .copied()
        .take(5)
        .enumerate()
        .try_fold(0u32, |result, (i, byte)| {
            let value = result | (((byte & 0x7F) as u32) << (i * 7));
            if byte & 0x80 == 0 {
                Err((value, i + 1))
            } else {
                Ok(value)
            }
        })
        .err()
}

/// Parses a length-prefixed slice where the length is encoded as varint32.
///
/// Returns `None` if:
/// - the varint32 prefix is malformed/incomplete, or
/// - the declared length exceeds available bytes.
pub fn get_length_prefixed_slice(data: &[u8]) -> Option<Slice<'_>> {
    let (len, prefix_len) = get_varint32(data)?;
    let end = prefix_len.checked_add(len as usize)?;
    Some(Slice::new(data.get(prefix_len..end)?))
}
```