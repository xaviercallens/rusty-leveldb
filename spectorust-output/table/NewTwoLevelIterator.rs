```rust
/// Creates a new two-level iterator from an index iterator, block factory, and read options.
pub fn new_two_level_iterator(
    index_iter: Box<dyn IteratorTrait>,
    block_function: BlockFunction,
    arg: *mut core::ffi::c_void,
    options: ReadOptions,
) -> Box<TwoLevelIterator> {
    Box::new(TwoLevelIterator::new(index_iter, block_function, arg, options))
}
```