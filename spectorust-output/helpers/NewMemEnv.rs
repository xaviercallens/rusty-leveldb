```rust
pub trait Env {}

pub struct InMemoryEnv {
    _base_env: Box<dyn Env>,
}

impl InMemoryEnv {
    pub fn new(base_env: Box<dyn Env>) -> Self {
        Self {
            _base_env: base_env,
        }
    }
}

/// Creates a new in-memory environment wrapping the provided base environment.
pub fn new_mem_env(base_env: Box<dyn Env>) -> Box<dyn Env> {
    Box::new(InMemoryEnv::new(base_env))
}

impl Env for InMemoryEnv {}
```