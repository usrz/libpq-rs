use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

napi_type!(NapiUndefined, Undefined, {
  handle: napi::Handle,
});

impl NapiTypeInternal for NapiUndefined {
  fn from_handle(handle: napi::Handle) -> Self {
    Self { handle }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== UNDEFINED =============================================================

impl NapiUndefined {
  pub fn new(env: napi::Env) -> Self {
    Self { handle: env.get_undefined() }
  }
}
