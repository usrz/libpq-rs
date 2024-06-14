use crate::napi;
use crate::types::*;

pub struct NapiString {
  handle: napi::Handle,
  value: String,
}

// ===== NAPI TYPE BASICS ======================================================

napi_value!(NapiString, String);

impl NapiTypeInternal for NapiString {
  fn from_handle(handle: napi::Handle) -> Self {
    Self { handle, value: handle.get_value_string_utf8() }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== CONVERSION OUT ========================================================

impl NapiString {
  pub fn value(&self) -> &str {
    &self.value
  }
}

// ===== CONVERSION IN =========================================================

impl <'a> NapiFrom<'a, &str> for NapiRef<'a, NapiString> {
  fn napi_from(value: &str, env: napi::Env) -> Self {
    NapiString {
      handle: env.create_string_utf8(&value),
      value: value.to_owned(),
    }.into()
  }
}
