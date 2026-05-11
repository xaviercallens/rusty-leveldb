```rust
/// Computes a 32-bit hash for the given byte slice, similar to MurmurHash.
///
/// This matches the behavior of the provided C++ implementation:
/// - Processes input in 4-byte little-endian chunks
/// - Mixes remaining 1..=3 tail bytes
/// - Uses wrapping arithmetic to mirror `uint32_t` overflow semantics
pub fn hash(data: &[u8], seed: u32) -> u32 {
    const M: u32 = 0xc6a4_a793;
    const R: u32 = 24;

    let mut h = seed ^ ((data.len() as u32).wrapping_mul(M));

    let mut chunks = data.chunks_exact(4);
    for chunk in &mut chunks {
        let w = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        h = h.wrapping_add(w);
        h = h.wrapping_mul(M);
        h ^= h >> 16;
    }

    let rem = chunks.remainder();
    if !rem.is_empty() {
        let tail = rem
            .iter()
            .enumerate()
            .fold(0u32, |acc, (idx, &b)| acc.wrapping_add((b as u32) << (idx * 8)));
        h = h.wrapping_add(tail);
        h = h.wrapping_mul(M);
        h ^= h >> R;
    }

    h
}
```