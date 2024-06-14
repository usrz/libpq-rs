use crate::napi;
use crate::types::*;

pub struct NapiBoolean {
  handle: napi::Handle,
  value: bool,
}

// ===== NAPI TYPE BASICS ======================================================

napi_type!(NapiBoolean, Boolean);

impl NapiTypeInternal for NapiBoolean {
  #[inline]
  fn from_handle(handle: napi::Handle) -> Self {
    Self { handle, value: handle.get_value_bool() }
  }

  #[inline]
  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== BOOLEAN ===============================================================

impl NapiBoolean {
  pub fn new(env: napi::Env, value: bool) -> Self {
    Self { handle: env.get_boolean(value), value }
  }

  pub fn value(&self) -> bool {
    self.value
  }
}
