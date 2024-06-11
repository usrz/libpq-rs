use super::*;

use nodejs_sys::*;
use std::mem::MaybeUninit;
use std::os::raw;
use std::ptr;

impl <'a> Env<'a> {
  pub fn is_exception_pending(&self) -> bool {
    unsafe {
      let mut result = MaybeUninit::<bool>::zeroed();
      env_check!(
        napi_is_exception_pending,
        self,
        result.as_mut_ptr()
      );
      result.assume_init()
    }
  }

  pub fn create_error(&self, message: &str) -> Handle<'a> {
    unsafe {
      let message = self.create_string_utf8(message);

      let mut result: MaybeUninit<nodejs_sys::napi_value> = MaybeUninit::zeroed();
      env_check!(
        napi_create_error,
        self,
        ptr::null_mut(),
        message.value,
        result.as_mut_ptr()
      );

      Handle { env: *self, value: result.assume_init() }
    }
  }

  pub fn throw(&self, handle: &Handle) {
    unsafe {
      let status = napi_throw(self.env, handle.value);
      if status == napi_status::napi_ok {
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
}

impl <'a> Handle<'a> {
  pub fn throw(&self, ) {
    self.env.throw(self)
  }
}
