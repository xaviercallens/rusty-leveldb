```rust
use windows_sys::Win32::Foundation::LocalFree;
use windows_sys::Win32::System::Diagnostics::Debug::{
    FormatMessageA, FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM,
    FORMAT_MESSAGE_IGNORE_INSERTS,
};
use windows_sys::Win32::System::SystemServices::{LANG_NEUTRAL, SUBLANG_DEFAULT};

/// Returns the system-provided message string for a Windows error code.
///
/// If no message can be retrieved, returns an empty string.
pub fn get_windows_error_message(error_code: u32) -> String {
    let mut buffer_ptr: *mut u8 = core::ptr::null_mut();
    let flags =
        FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_IGNORE_INSERTS;
    let lang_id = ((SUBLANG_DEFAULT as u32) << 10) | (LANG_NEUTRAL as u32);

    let size = unsafe {
        FormatMessageA(
            flags,
            core::ptr::null(),
            error_code,
            lang_id,
            (&mut buffer_ptr as *mut *mut u8).cast(),
            0,
            core::ptr::null_mut(),
        )
    };

    if size == 0 || buffer_ptr.is_null() {
        return String::new();
    }

    let message = unsafe {
        let bytes = core::slice::from_raw_parts(buffer_ptr, size as usize);
        String::from_utf8_lossy(bytes).trim_end().to_owned()
    };

    unsafe {
        LocalFree(buffer_ptr as isize);
    }

    message
}
```