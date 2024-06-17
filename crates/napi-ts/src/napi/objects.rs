use crate::napi::*;

impl Env {

  pub fn create_object(&self) -> Handle {
    unsafe {
      let mut result: MaybeUninit<napi_value> = MaybeUninit::zeroed();
      env_check!(
        napi_create_object,
        self,
        result.as_mut_ptr()
      );
      Handle(result.assume_init())
    }
  }

  /* ======================================================================== *
   * PROPERTIES                                                               *
   * ======================================================================== */

  pub fn set_named_property(&self, object: &Handle, key: &str, value: &Handle) {
    unsafe {
      // here we use "napi_set_property" to get the same results of
      // "napi_set_named_property" but we don't deal with null-terminating
      // strings, as "napi_set_named_property" doesn't accept "length"
      let key = self.create_string_utf8(key);

      env_check!(
        napi_set_property,
        self,
        object.0,
        key.0,
        value.0
      );
    }
  }

  pub fn get_named_property(&self, object: &Handle, key: &str) -> Handle {
    unsafe {
      // here we use "napi_get_property" to get the same results of
      // "napi_get_named_property" but we don't deal with null-terminating
      // strings, as "napi_get_named_property" doesn't accept "length"
      let key = self.create_string_utf8(key);

      let mut result: MaybeUninit<napi_value> = MaybeUninit::zeroed();
      env_check!(
        napi_get_property,
        self,
        object.0,
        key.0,
        result.as_mut_ptr()
      );
      Handle(result.assume_init())
    }
  }
}

impl Handle {
  pub fn set_named_property(&self, key: &str, value: &Handle) {
    env().set_named_property(self, key, value)
  }
  pub fn get_named_property(&self, key: &str) -> Handle {
    env().get_named_property(self, key)
  }
}
