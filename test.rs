#![crate_type = "staticlib"]
//#![no_std]
#![allow(ctypes)]

//#![feature(lang_items)]
//#[lang="sized"]
//trait Sized {}

#[no_mangle]
pub extern "C" fn add(lhs: i32, rhs: i32) -> i32 {
  lhs + rhs
}

