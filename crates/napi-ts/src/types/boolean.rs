use crate::napi;
use crate::types::*;

pub struct NapiBoolean<'a> {
  handle: NapiHandle<'a>,
  value: bool,
}

impl Debug for NapiBoolean<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiBoolean")
      .field("@", &self.handle.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiBoolean<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiBoolean<'a> {
  fn from_napi_handle(handle: NapiHandle<'a>) -> Result<Self, NapiErr> {
    napi::expect_type_of(handle.env, handle.handle, napi::TypeOf::napi_boolean)
      .map(|_| Self::from_napi_handle_unchecked(handle))
  }

  fn from_napi_handle_unchecked(handle: NapiHandle<'a>) -> Self {
    let value = napi::get_value_bool(handle.env, handle.handle);
    Self { handle, value }
  }

  fn get_napi_handle(&self) -> &NapiHandle<'a> {
    &self.handle
  }
}

// ===== BOOL ==================================================================

impl NapiFrom<bool> for NapiBoolean<'_> {
  fn napi_from(value: bool, env: napi::Env) -> Self {
    let handle = napi::get_boolean(env, value);
    Self { handle: NapiHandle::from_napi(env, handle), value }
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
