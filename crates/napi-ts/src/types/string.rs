use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiString<'a> {
  phantom: PhantomData<&'a ()>,
  handle: napi::Handle,
  value: String,
}

napi_type!(NapiString, String, {
  unsafe fn from_handle(handle: napi::Handle) -> Result<Self, NapiErr> {
    Ok(Self { phantom: PhantomData, handle, value: handle.get_value_string_utf8() })
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});

// ===== STRING ================================================================

impl <'a> NapiString<'a> {
  pub fn new(value: &str) -> Self {
    Self {
      phantom: PhantomData,
      handle: napi::env().create_string_utf8(&value),
      value: value.to_owned(),
    }
  }

  pub fn value(&self) -> &str {
    &self.value
  }
}
