use crate::napi;
use crate::types::*;

pub struct NapiBoolean<'a> {
  handle: napi::Handle<'a>,
  value: bool,
}

impl Debug for NapiBoolean<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiBoolean")
      .field("@", &self.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiBoolean<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiBoolean<'a> {
  fn from_napi_handle(handle: napi::Handle<'a>) -> Result<Self, NapiErr> {
    handle.expect_type_of(napi::TypeOf::napi_boolean)
      .map(|_| Self::from_napi_handle_unchecked(handle))
  }

  fn from_napi_handle_unchecked(handle: napi::Handle<'a>) -> Self {
    let value = handle.get_value_bool();
    Self { handle, value }
  }

  fn napi_handle(&self) -> napi::Handle<'a> {
    self.handle
  }
}

// ===== BOOL ==================================================================

impl <'a> NapiFrom<'a, bool> for NapiBoolean<'a> {
  fn napi_from(value: bool, env: napi::Env<'a>) -> Self {
    Self { handle: env.get_boolean(value), value }
  }
}

impl Into<bool> for NapiBoolean<'_> {
  fn into(self) -> bool {
    self.value
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiBoolean<'_> {
  pub fn value(&self) -> bool {
    self.value
  }
}
