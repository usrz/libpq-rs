use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

napi_type!(NapiNull, Null, {
  handle: napi::Handle,
});

impl NapiTypeInternal for NapiNull {
  #[inline]
  fn from_handle(handle: napi::Handle) -> Self {
    Self { handle }
  }

  #[inline]
  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== NULL ==================================================================

impl NapiNull {
  pub fn new(env: napi::Env) -> Self {
    Self { handle: env.get_null() }
  }
}
