use crate::types::*;
use std::cell::OnceCell;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiArray {
  handle: napi::Handle,
  push: OnceCell<napi::Handle>,
  pop: OnceCell<napi::Handle>,
}

napi_type!(NapiArray, Object, {
  unsafe fn from_handle(handle: napi::Handle) -> Result<Self, NapiErr> {
    if ! handle.is_array() {
      return Err("Specified object is not a JavaScript Array".into())
    } else {
      Ok(Self {
        handle,
        push: OnceCell::new(),
        pop: OnceCell::new(),
      })
    }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});

impl <'a> NapiProperties<'a> for NapiRef<'a, NapiArray> {}

// ===== ARRAY =================================================================

impl NapiArray {
  pub fn new(env: napi::Env) -> Self {
    unsafe { Self::from_handle(env.create_array()).unwrap() }
  }
}

impl <'a> NapiRef<'a, NapiArray> {
  pub fn length(&self) -> u32 {
    let value = self.handle.get_named_property("length");
    value.get_value_double() as u32
  }

  pub fn get_element(&self, index: u32) -> NapiRef<'a, NapiValue> {
    let value = self.handle.get_element(index);
    NapiValue::from_handle(value).as_napi_ref()
  }

  pub fn set_element<T: NapiType + 'a>(
    &self, index: u32, value: &NapiRef<'a, T>
  ) -> &Self {
    self.handle.set_element(index, &value.napi_handle());
    self
  }

  pub fn has_element(&self, index: u32) -> bool {
    self.handle.has_element(index)
  }

  pub fn delete_element(&self, index: u32) {
    self.handle.delete_element(index)
  }

  pub fn push<T: NapiType + 'a>(&self, value: &NapiRef<'a, T>) -> u32 {
    let push = self.value.push.get_or_init(|| self.handle.get_named_property("push"));
    let result = push.call_function(&self.handle, &[&value.napi_handle()]).unwrap();
    self.handle.env().get_value_double(&result) as u32
  }

  pub fn pop<T: NapiType + 'a>(&self) -> u32 {
    let push = self.value.pop.get_or_init(|| self.handle.get_named_property("pop"));
    let result = push.call_function(&self.handle, &[]).unwrap();
    self.handle.env().get_value_double(&result) as u32
  }
}
