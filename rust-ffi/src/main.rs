// We use extern "C" to declare functions that follow C's calling convention.
// The function signatures must match the C declarations exactly.
unsafe extern "C" {
    fn strlen(s: *const std::os::raw::c_char) -> usize;
}

fn main() {
    // CString handles null-termination for us - C strings require a trailing \0
    let rust_string = std::ffi::CString::new("Hello, FFI!").unwrap();

    // We must use unsafe because the compiler cannot verify the C function's behavior
    let length = unsafe { strlen(rust_string.as_ptr()) };

    println!("String length: {}", length);
}
