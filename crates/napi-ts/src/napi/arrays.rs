use crate::napi::*;

impl Env {
  pub fn is_array(&self, handle: &Handle) -> bool {
    unsafe {
      let mut result: MaybeUninit<bool> = MaybeUninit::zeroed();
      env_check!(
        napi_is_array,
        self,
        handle.value,
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
      self.handle(result.assume_init())
    }
  }

  pub fn set_element(&self, object: &Handle, index: u32, value: &Handle ) {
    unsafe {
      env_check!(
        napi_set_element,
        self,
        object.value,
        index,
        value.value
      )
    }
  }

  pub fn get_element(&self, object: &Handle, index: u32) -> Handle {
    unsafe {
      let mut result: MaybeUninit<napi_value> = MaybeUninit::zeroed();
      env_check!(
        napi_get_element,
        self,
        object.value,
        index,
        result.as_mut_ptr()
      );
      self.handle(result.assume_init())
    }
  }

  pub fn has_element(&self, object: &Handle, index: u32) -> bool {
    unsafe {
      let mut result: MaybeUninit<bool> = MaybeUninit::zeroed();
      env_check!(
        napi_has_element,
        self,
        object.value,
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
        object.value,
        index,
        ptr::null_mut()
      );
    }
  }
}

impl Handle {
  pub fn is_array(&self) -> bool {
    self.env.is_array(self)
  }
  pub fn set_element(&self, index: u32, value: &Handle ) {
    self.env.set_element(self, index, value)
  }
  pub fn get_element(&self, index: u32) -> Handle {
    self.env.get_element(self, index)
  }
  pub fn has_element(&self, index: u32) -> bool {
    self.env.has_element(self, index)
  }
  pub fn delete_element(&self, index: u32) {
    self.env.delete_element(self, index)
  }
}
