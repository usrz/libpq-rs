use crate::types::*;

pub struct NapiBigint {
  handle: napi::Handle,
  value: i128,
}

// ===== NAPI TYPE BASICS ======================================================

napi_type!(NapiBigint, Bigint);

impl NapiTypeInternal for NapiBigint {
  #[inline]
  fn from_handle(handle: napi::Handle) -> Self {
    Self { handle, value: handle.get_value_bigint_words() }
  }

  #[inline]
  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== BIGINT ================================================================

impl NapiBigint {
  pub fn new(env: napi::Env, value: i128) -> Self {
    Self { handle: env.create_bigint_words(value), value }
  }

  pub fn value(&self) -> i128 {
    self.value
  }
}
