use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiBigint {
  handle: napi::Handle,
  value: i128,
}

napi_type!(NapiBigint, Bigint, {
  unsafe fn from_handle(handle: napi::Handle) -> Result<Self, NapiErr> {
    Ok(Self { handle, value: handle.get_value_bigint_words() })
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});


// ===== BIGINT ================================================================

impl NapiBigint {
  pub fn new(env: napi::Env, value: i128) -> Self {
    Self { handle: env.create_bigint_words(value), value }
  }

  pub fn value(&self) -> i128 {
    self.value
  }
}
