use std::ffi::{CStr, c_char, c_int, c_uint};

#[repr(C)]
pub struct rtlsdr_dev_t {
    _private: [u8; 0], // 불투명 구조체 (opaque type)
}

#[link(name = "rtlsdr")]
unsafe extern "C" {
    fn rtlsdr_get_device_count() -> c_uint;
    fn rtlsdr_get_device_name(index: c_uint) -> *const c_char;
    fn rtlsdr_open(dev: *mut *mut rtlsdr_dev_t, index: c_uint) -> c_int;
    fn rtlsdr_close(dev: *mut *mut rtlsdr_dev_t) -> c_int;
}

fn main() {
    println!("Hello, world!");

    unsafe {
        println!("{}", rtlsdr_get_device_count());

        let c_str = CStr::from_ptr(rtlsdr_get_device_name(0));

        let str = c_str.to_str().unwrap();

        println!("{}", str);

        let mut dev: *mut rtlsdr_dev_t = std::ptr::null_mut();
        if rtlsdr_open(&mut dev as *mut *mut rtlsdr_dev_t, 0) != 0 {
            println!("error rtlsdr_open {}", 0);
        }

        if rtlsdr_close(dev as *mut *mut rtlsdr_dev_t) != 0 {
            println!("error rtlsdr_close {}", 0);
        }
    }
}
