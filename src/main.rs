#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::env;
use std::ffi::CString;

fn main() {
    let mut args = env::args();
    let source = CString::new(args.nth(1).unwrap()).unwrap();
    let dest = CString::new(args.next().unwrap()).unwrap();

    unsafe {
        let data = libraw_init(0);
        libraw_open_file(data, source.as_ptr());
        libraw_unpack_thumb(data);
        libraw_dcraw_thumb_writer(data, dest.as_ptr());
    }
}
