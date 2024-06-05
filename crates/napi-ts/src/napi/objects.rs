use super::*;

use nodejs_sys::*;
use std::mem::MaybeUninit;

pub fn create_object() -> Value {
  unsafe {
    let mut result = MaybeUninit::<Value>::zeroed();
    napi_check!(napi_create_object, result.as_mut_ptr());
    result.assume_init()
  }
}

/* ========================================================================== *
 * PROPERTIES                                                                 *
 * ========================================================================== */

pub fn set_property(object: Value, key: Value, value: Value) {
  unsafe {
    napi_check!(napi_set_property, object, key, value);
  }
}

pub fn get_property(object: Value, key: Value) -> Value {
  unsafe {
    let mut result = MaybeUninit::<Value>::zeroed();
    napi_check!(napi_get_property, object, key, result.as_mut_ptr());
    result.assume_init()
  }
}
