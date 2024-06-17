use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiBoolean<'a> {
  phantom: PhantomData<&'a ()>,
  handle: napi::Handle,
  value: bool,
}

napi_type!(NapiBoolean, Boolean, {
  unsafe fn from_handle(handle: napi::Handle) -> Result<Self, NapiErr> {
    Ok(Self { phantom: PhantomData, handle, value: handle.get_value_bool() })
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});

// ===== BOOLEAN ===============================================================

impl <'a> NapiBoolean<'a> {
  pub fn new(value: bool) -> Self {
    Self { phantom: PhantomData, handle: napi::env().get_boolean(value), value }
  }

  pub fn value(&self) -> bool {
    self.value
  }
}
