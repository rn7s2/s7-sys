#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::string::FromUtf8Error;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn char_arr_to_string(arr: &[u8]) -> Result<String, FromUtf8Error> {
    String::from_utf8(arr.to_owned())
}

pub fn char_pointer_to_string(ptr: *mut i8) -> Result<String, FromUtf8Error> {
    let mut len = 0;
    while unsafe { *ptr.add(len) } != 0 {
        len += 1;
    }
    let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, len) };
    char_arr_to_string(slice)
}
