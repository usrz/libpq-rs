use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiUndefined {
  handle: napi::Handle,
}

napi_type!(NapiUndefined, Undefined, {
  unsafe fn from_handle(handle: napi::Handle) -> Self {
    Self { handle }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});

// ===== UNDEFINED =============================================================

impl NapiUndefined {
  pub fn new(env: napi::Env) -> Self {
    Self { handle: env.get_undefined() }
  }
}
