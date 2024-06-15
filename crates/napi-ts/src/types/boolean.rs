use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiBoolean {
  handle: napi::Handle,
  value: bool,
}

napi_type!(NapiBoolean, Boolean, {
  unsafe fn from_handle(handle: napi::Handle) -> Self {
    Self { handle, value: handle.get_value_bool() }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});

// ===== BOOLEAN ===============================================================

impl NapiBoolean {
  pub fn new(env: napi::Env, value: bool) -> Self {
    Self { handle: env.get_boolean(value), value }
  }

  pub fn value(&self) -> bool {
    self.value
  }
}
