use crate::napi;
use crate::types::*;

pub struct NapiNull {
  handle: napi::Handle,
}

// ===== NAPI TYPE BASICS ======================================================

// napi_type!(NapiNull, Null);
napi_type!(NapiNull, Null);

impl NapiTypeInternal for NapiNull {
  fn from_handle(handle: napi::Handle) -> Self {
    Self { handle }
  }

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
