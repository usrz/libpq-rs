use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

napi_type!(NapiString, String, {
  handle: napi::Handle,
  value: String,
});

impl NapiTypeInternal for NapiString {
  #[inline]
  fn from_handle(handle: napi::Handle) -> Self {
    Self { handle, value: handle.get_value_string_utf8() }
  }

  #[inline]
  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== STRING ================================================================

impl NapiString {
  pub fn new(env: napi::Env, value: &str) -> Self {
    Self {
      handle: env.create_string_utf8(&value),
      value: value.to_owned(),
    }
  }

  pub fn value(&self) -> &str {
    &self.value
  }
}
