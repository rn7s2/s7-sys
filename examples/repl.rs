extern crate s7_sys as s7;

use std::{ffi::CString, io::Write};

fn main() {
    unsafe {
        print_s7_version();
        let sc = s7::s7_init();
        s7::s7_repl(sc);
    }
}

fn print_s7_version() {
    println!(
        "s7 version: {}",
        CString::new(s7::S7_VERSION[..s7::S7_VERSION.len() - 1].to_vec())
            .expect("Convert CString to Rust String error")
            .to_str()
            .expect("Convert CString to Rust String error")
    );
    std::io::stdout().flush().expect("Flush stdout error");
}
