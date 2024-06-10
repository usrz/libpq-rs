use crate::napi;
use crate::types::*;

pub struct NapiString<'a> {
  handle: NapiHandle<'a>,
  value: String,
}

impl Debug for NapiString<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiString")
      .field("@", &self.handle.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiString<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiString<'a> {
  fn from_napi_handle(handle: NapiHandle<'a>) -> Result<Self, NapiErr> {
    napi::expect_type_of(handle.env, handle.handle, napi::TypeOf::napi_string)
      .map(|_| Self::from_napi_handle_unchecked(handle))
  }

  fn from_napi_handle_unchecked(handle: NapiHandle<'a>) -> Self {
    let value = napi::get_value_string_utf8(handle.env, handle.handle);
    Self { handle, value }
  }

  fn get_napi_handle(&self) -> &NapiHandle<'a> {
    &self.handle
  }
}

// ===== STRING ================================================================

impl NapiFrom<&str> for NapiString<'_> {
  fn napi_from(value: &str, env: napi::Env) -> Self {
    let handle = napi::create_string_utf8(env, value);
    Self { handle: NapiHandle::from_napi(env, handle), value: value.to_string() }
  }
}

impl Into<String> for NapiString<'_> {
  fn into(self) -> String {
    self.value
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiString<'_> {
  pub fn value(&self) -> String {
    self.value.clone()
  }
}
