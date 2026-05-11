```rust
pub trait Cache {}

pub struct ShardedLruCache {
    capacity: usize,
}

impl ShardedLruCache {
    pub fn new(capacity: usize) -> Self {
        Self { capacity }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl Cache for ShardedLruCache {}

pub fn new_lru_cache(capacity: usize) -> Box<dyn Cache> {
    Box::new(ShardedLruCache { capacity })
}
```