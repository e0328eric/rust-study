#[no_mangle]
pub extern "C" fn get_string() -> *const u8 {
    b"Hello, C-World!\n\0".as_ptr()
}
