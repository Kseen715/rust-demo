use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern fn print_rs() {
    println!("Hello from Rust's DLL!");
}

#[no_mangle]
pub extern fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[no_mangle]
pub extern fn print_str(s: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(s) };
    let str_slice = c_str.to_str().unwrap();
    println!("{}", str_slice);
}

#[no_mangle]
pub extern fn pow_rs(x: f64, y: i32) -> f64 {
    let mut result = 1.0;
    for _ in 0..y {
        result *= x;
    }
    result
}