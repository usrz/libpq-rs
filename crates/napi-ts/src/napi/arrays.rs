use crate::napi::*;

impl Env {
  pub fn is_array(&self, handle: &Handle) -> bool {
    unsafe {
      let mut result: MaybeUninit<bool> = MaybeUninit::zeroed();
      env_check!(
        napi_is_array,
        self,
        handle.0,
        result.as_mut_ptr()
      );
      result.assume_init()
    }

  }

  pub fn create_array(&self) -> Handle {
    unsafe {
      let mut result: MaybeUninit<napi_value> = MaybeUninit::zeroed();
      env_check!(
        napi_create_array,
        self,
        result.as_mut_ptr()
      );
      Handle(result.assume_init())
    }
  }

  pub fn set_element(&self, object: &Handle, index: u32, value: &Handle ) {
    unsafe {
      env_check!(
        napi_set_element,
        self,
        object.0,
        index,
        value.0
      )
    }
  }

  pub fn get_element(&self, object: &Handle, index: u32) -> Handle {
    unsafe {
      let mut result: MaybeUninit<napi_value> = MaybeUninit::zeroed();
      env_check!(
        napi_get_element,
        self,
        object.0,
        index,
        result.as_mut_ptr()
      );
      Handle(result.assume_init())
    }
  }

  pub fn has_element(&self, object: &Handle, index: u32) -> bool {
    unsafe {
      let mut result: MaybeUninit<bool> = MaybeUninit::zeroed();
      env_check!(
        napi_has_element,
        self,
        object.0,
        index,
        result.as_mut_ptr()
      );
      result.assume_init()
    }
  }

  pub fn delete_element(&self, object: &Handle, index: u32) {
    unsafe {
      env_check!(
        napi_delete_element,
        self,
        object.0,
        index,
        ptr::null_mut()
      );
    }
  }
}

impl Handle {
  pub fn is_array(&self) -> bool {
    env().is_array(self)
  }
  pub fn set_element(&self, index: u32, value: &Handle ) {
    env().set_element(self, index, value)
  }
  pub fn get_element(&self, index: u32) -> Handle {
    env().get_element(self, index)
  }
  pub fn has_element(&self, index: u32) -> bool {
    env().has_element(self, index)
  }
  pub fn delete_element(&self, index: u32) {
    env().delete_element(self, index)
  }
}
