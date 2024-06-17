use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiNumber<'a> {
  phantom: PhantomData<&'a ()>,
  handle: napi::Handle,
  value: f64,
}

napi_type!(NapiNumber, Number, {
  unsafe fn from_handle(handle: napi::Handle) -> Result<Self, NapiErr> {
    Ok(Self { phantom: PhantomData, handle, value: handle.get_value_double() })
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});

// ===== NUMBER ================================================================

impl <'a> NapiNumber<'a> {
  pub fn new(value: f64) -> Self {
    Self { phantom: PhantomData, handle: napi::env().create_double(value), value }
  }

  pub fn value(&self) -> f64 {
    self.value
  }
}
