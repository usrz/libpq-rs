use crate::env::Napi;
use super::*;

use nodejs_sys::*;
use std::mem::MaybeUninit;
use std::os::raw;
use std::ptr;

pub fn is_exception_pending() -> bool {
  unsafe {
    let mut result = MaybeUninit::<bool>::zeroed();
    napi_check!(napi_is_exception_pending, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn create_error(message: String) -> Handle {
  unsafe {
    let message = create_string_utf8(&message);

    let mut result = MaybeUninit::<Handle>::zeroed();
    napi_check!(napi_create_error, ptr::null_mut(), message, result.as_mut_ptr());

    result.assume_init()
  }
}

pub fn throw(error: Handle) {
  unsafe {
    let status = napi_throw(Napi::env(), error);
    if status == Status::napi_ok {
      return
    }

    let location = format!("{} line {}", file!(), line!());
    let message = format!("Error throwing (status={:?})", status);
    nodejs_sys::napi_fatal_error(
      location.as_ptr() as *const raw::c_char,
      location.len(),
      message.as_ptr() as *const raw::c_char,
      message.len());
  }
}
