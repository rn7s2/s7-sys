use s7::char_arr_to_string;

extern crate s7_sys as s7;

fn main() {
    unsafe {
        print_s7_version();
        let sc = s7::s7_init();
        s7::s7_repl(sc);
    }
}

fn print_s7_version() {
    let ver = char_arr_to_string(s7::S7_VERSION).unwrap();
    let date = char_arr_to_string(s7::S7_DATE).unwrap();
    println!("s7 version: {ver}, {date}");
}
