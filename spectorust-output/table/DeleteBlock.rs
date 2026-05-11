```rust
/// Deletes a `Block` previously allocated on the heap.
///
/// # Safety
/// - `arg` must be a valid pointer obtained from `Box::into_raw(Box<Block>)`.
/// - `arg` must not be null.
/// - `arg` must not have been freed already.
/// - `_ignored` is intentionally unused.
pub unsafe fn delete_block(arg: *mut core::ffi::c_void, _ignored: *mut core::ffi::c_void) {
    if arg.is_null() {
        return;
    }
    drop(unsafe { Box::from_raw(arg.cast::<Block>()) });
}
```