use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

napi_type!(NapiNumber, Number, {
  handle: napi::Handle,
  value: f64,
});

impl NapiTypeInternal for NapiNumber {
  #[inline]
  fn from_handle(handle: napi::Handle) -> Self {
    Self { handle, value: handle.get_value_double() }
  }

  #[inline]
  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== NUMBER ================================================================

impl NapiNumber {
  pub fn new(env: napi::Env, value: f64) -> Self {
    Self { handle: env.create_double(value), value }
  }

  pub fn value(&self) -> f64 {
    self.value
  }
}
