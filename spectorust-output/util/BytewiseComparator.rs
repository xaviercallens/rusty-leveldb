```rust
use std::sync::OnceLock;

pub trait Comparator {
    fn compare(&self, a: &[u8], b: &[u8]) -> std::cmp::Ordering;
}

#[derive(Debug, Default)]
pub struct BytewiseComparatorImpl;

impl Comparator for BytewiseComparatorImpl {
    fn compare(&self, a: &[u8], b: &[u8]) -> std::cmp::Ordering {
        a.cmp(b)
    }
}

pub fn bytewise_comparator() -> &'static dyn Comparator {
    static SINGLETON: OnceLock<BytewiseComparatorImpl> = OnceLock::new();
    SINGLETON.get_or_init(BytewiseComparatorImpl::default) as &dyn Comparator
}
```