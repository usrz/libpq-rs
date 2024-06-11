use crate::napi;
use crate::types::*;

pub struct NapiString<'a> {
  handle: napi::Handle<'a>,
  value: String,
}

impl Debug for NapiString<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiString")
      .field("@", &self.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiString<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiString<'a> {
  fn from_napi_handle(handle: napi::Handle<'a>) -> Result<Self, NapiErr> {
    handle.expect_type_of(napi::TypeOf::napi_string)
      .map(|_| Self::from_napi_handle_unchecked(handle))
  }

  fn from_napi_handle_unchecked(handle: napi::Handle<'a>) -> Self {
    let value = handle.get_value_string_utf8();
    Self { handle, value }
  }

  fn napi_handle(&self) -> napi::Handle<'a> {
    self.handle
  }
}

// ===== STRING ================================================================

impl <'a> NapiFrom<'a, &str> for NapiString<'a> {
  fn napi_from(value: &str, env: napi::Env<'a>) -> Self {
    let handle = env.create_string_utf8(value);
    Self { handle, value: value.to_string() }
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
