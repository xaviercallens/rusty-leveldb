```rust
#[inline]
pub fn random_seed() -> i32 {
    testing::unit_test::get_instance().random_seed()
}
```