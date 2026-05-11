```rust
/// Computes the Bloom filter hash for a key slice using a fixed seed.
///
/// Mirrors the C++ implementation:
/// `Hash(key.data(), key.size(), 0xbc9f1d34)`.
#[inline]
pub fn bloom_hash(key: &[u8]) -> u32 {
    hash(key, 0xbc9f1d34)
}
```