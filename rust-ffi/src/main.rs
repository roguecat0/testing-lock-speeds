// We use extern "C" to declare functions that follow C's calling convention.
// The function signatures must match the C declarations exactly.

// use std::ffi::c_int;
unsafe extern "C" {
    // fn strlen(s: *const std::os::raw::c_char) -> usize;
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    // CString handles null-termination for us - C strings require a trailing \0
    // let rust_string = std::ffi::CString::new("Hello, FFI!").unwrap();

    // We must use unsafe because the compiler cannot verify the C function's behavior
    // let length = unsafe { strlen(rust_string.as_ptr()) };
    let res = unsafe { add(2, 3) };

    println!("String length: {}", res);
}
#[link(name = "math_ops")]
unsafe extern "C" {}
