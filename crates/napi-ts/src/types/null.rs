use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiNull {
  handle: napi::Handle,
}

napi_type!(NapiNull, Null, {
  unsafe fn from_handle(handle: napi::Handle) -> Result<Self, NapiErr> {
    Ok(Self { handle })
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});

// ===== NULL ==================================================================

impl NapiNull {
  pub fn new(env: napi::Env) -> Self {
    Self { handle: env.get_null() }
  }
}
