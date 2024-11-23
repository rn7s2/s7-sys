extern crate s7_sys as s7;

use std::{
    os::raw::c_char,
    sync::{Arc, Mutex},
    thread,
};

use s7::{
    char_arr_to_string, char_pointer_to_string, s7_eval_c_string, s7_init, s7_make_integer,
    s7_object_to_c_string, s7_scheme, S7_DATE, S7_VERSION,
};

const NUM_THREADS: usize = 16;

fn main() {
    print_s7_version();

    let mut threads = Vec::new();
    let lock = Arc::new(Mutex::new(0));
    for i in 0..NUM_THREADS {
        let sc = unsafe { s7_init() as usize };
        let lock = lock.clone();
        threads.push(thread::spawn(move || unsafe {
            let sc = sc as *mut s7_scheme;
            let str = char_pointer_to_string(s7_object_to_c_string(sc, s7_make_integer(sc, 123)))
                .unwrap();

            let src = "(let () \
                                   (define (f) \
                                     (do ((i 0 (+ i 1))) ((= i 10)) \
                                       (do ((k 0 (+ k 1))) ((= k 1000000))) \
                                       (format *stderr* \"~D \" i))) \
                                   (f) \
                                   (format *stderr* \"~%\")))";
            s7_eval_c_string(sc, src.as_ptr() as *const c_char);

            let _guard = lock.lock().unwrap();
            eprintln!("thread {}: {}", i, str);
        }));
    }

    for t in threads {
        t.join().unwrap();
    }
}

fn print_s7_version() {
    let ver = char_arr_to_string(S7_VERSION).unwrap();
    let date = char_arr_to_string(S7_DATE).unwrap();
    println!("s7 version: {ver}, {date}");
}
