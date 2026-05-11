```rust
pub fn repair_db(dbname: &str, options: &Options) -> Status {
    Repairer::new(dbname, options).run()
}
```