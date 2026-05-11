```rust
/// Clamps the value pointed to by `value` into the inclusive range `[min, max]`.
///
/// If `*value` is greater than `max`, it is set to `max`.
/// If `*value` is less than `min`, it is set to `min`.
pub fn clip_to_range<T>(value: &mut T, min: T, max: T)
where
    T: PartialOrd + Copy,
{
    *value = if *value > max {
        max
    } else if *value < min {
        min
    } else {
        *value
    };
}
```