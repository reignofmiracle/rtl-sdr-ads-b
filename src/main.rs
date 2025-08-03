use std::ffi::c_uint;

#[link(name = "rtlsdr")]
unsafe extern "C" {
    unsafe fn rtlsdr_get_device_count() -> c_uint;
}

fn main() {
    println!("Hello, world!");

    unsafe {
        println!("", rtlsdr_get_device_count());
    }
}
