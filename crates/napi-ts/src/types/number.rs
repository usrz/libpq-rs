use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiNumber {
  handle: napi::Handle,
  value: f64,
}

napi_type!(NapiNumber, Number, {
  unsafe fn from_handle(handle: napi::Handle) -> Self {
    Self { handle, value: handle.get_value_double() }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});

// ===== NUMBER ================================================================

impl NapiNumber {
  pub fn new(env: napi::Env, value: f64) -> Self {
    Self { handle: env.create_double(value), value }
  }

  pub fn value(&self) -> f64 {
    self.value
  }
}
