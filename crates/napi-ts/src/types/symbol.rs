
use crate::napi;
use crate::types::*;
use std::ptr;

pub struct NapiSymbol<'a> {
  handle: NapiHandle<'a>,
}

impl Debug for NapiSymbol<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiSymbol")
      .field("@", &self.handle.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiSymbol<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiSymbol<'a> {
  fn from_napi_handle(handle: NapiHandle<'a>) -> Result<Self, NapiErr> {
    napi::expect_type_of(handle.env, handle.handle, napi::TypeOf::napi_symbol)
      .map(|_| Self::from_napi_handle_unchecked(handle))
  }

  fn from_napi_handle_unchecked(handle: NapiHandle<'a>) -> Self {
    Self { handle }
  }

  fn get_napi_handle(&self) -> &NapiHandle<'a> {
    &self.handle
  }
}

// ===== STRING ================================================================

impl NapiFrom<Option<&str>> for NapiSymbol<'_> {
  fn napi_from(value: Option<&str>, env: napi::Env) -> Self {
    let description = match value {
      Some(description) => napi::create_string_utf8(env, description),
      None => ptr::null_mut(),
    };

    let handle = napi::create_symbol(env, description);
    Self { handle: NapiHandle::from_napi(env, handle) }
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiSymbol<'_> {
  pub fn description(&self) -> Option<String> {
    let env = self.handle.env;
    let key = napi::create_string_utf8(env, "description");
    let value = napi::get_property(env, self.handle.handle, key);

    match napi::type_of(self.handle.env, value) {
      napi::TypeOf::napi_string => Some(napi::get_value_string_utf8(env, value)),
      _ => None,
    }
  }
}
