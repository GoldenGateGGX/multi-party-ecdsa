#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{c_char, CStr, CString};

// count int, group int, pallier_key_length int, keyType *C.char
pub fn create_node(count: i64, group: i64, pallier_key_length: i64, name: &str) -> String {
    let name = CString::new(name).unwrap();

    unsafe {
        let cstr = CStr::from_ptr(CreateNode(
            count,
            group,
            pallier_key_length,
            name.as_ptr() as *mut c_char,
        ));
        let s = String::from_utf8_lossy(cstr.to_bytes()).to_string();
        GoFree(cstr.as_ptr() as *mut c_char);
        s
    }
}
