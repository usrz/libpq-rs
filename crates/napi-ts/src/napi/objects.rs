use super::*;

use nodejs_sys::*;
use std::mem::MaybeUninit;

impl <'a> Env<'a> {

  pub fn create_object(&self) -> Handle<'a> {
    unsafe {
      let mut result: MaybeUninit<nodejs_sys::napi_value> = MaybeUninit::zeroed();
      env_check!(
        napi_create_object,
        self,
        result.as_mut_ptr()
      );
      Handle { env: *self, value: result.assume_init() }
    }
  }

  /* ========================================================================== *
  * PROPERTIES                                                                 *
  * ========================================================================== */

  pub fn set_property(&self, object: &Handle, key: &Handle, value: &Handle) {
    unsafe {
      env_check!(
        napi_set_property,
        self,
        object.value,
        key.value,
        value.value
      );
    }
  }

  pub fn get_property(&self, object: &Handle, key: &Handle) -> Handle<'a> {
    unsafe {
      let mut result: MaybeUninit<nodejs_sys::napi_value> = MaybeUninit::zeroed();
      env_check!(
        napi_get_property,
        self,
        object.value,
        key.value,
        result.as_mut_ptr()
      );
      Handle { env: *self, value: result.assume_init() }
    }
  }
}

impl <'a> Handle<'a> {
  pub fn set_property(&self, key: &Handle, value: &Handle) {
    self.env.set_property(self, key, value)
  }

  pub fn get_property(&'a self, key: &Handle) -> Handle<'a> {
    self.env.get_property(self, key)
  }
}
