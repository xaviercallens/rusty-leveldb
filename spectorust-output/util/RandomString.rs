```rust
use rand::Rng;

/// Generates a random printable ASCII string (`' '` .. `'~'`) of length `len`,
/// stores it into `dst`, and returns it as `&str`.
///
/// # Panics
/// Panics if `len` is larger than `isize::MAX` on platforms where allocation
/// limits are exceeded.
pub fn random_string<R: Rng + ?Sized>(rnd: &mut R, len: usize, dst: &mut String) -> &str {
    dst.clear();
    dst.reserve(len);
    dst.extend((0..len).map(|_| rnd.gen_range(b' '..=b'~') as char));
    dst
}
```