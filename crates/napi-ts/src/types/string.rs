use crate::napi;
use crate::types::*;

#[derive(Clone,Debug)]
pub struct NapiString {
  value: napi::Value,
}

impl NapiValue for NapiString {}

impl NapiValueInternal for NapiString {
  fn as_napi_value(&self) -> napi::Value {
    self.value
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { value }
  }
}

// ===== &STR CONVERSION =======================================================

impl From<&str> for NapiString {
  fn from(value: &str) -> Self {
    Self { value: napi::create_string_utf8(value) }
  }
}

// ===== STRING CONVERSION =====================================================

impl From<String> for NapiString {
  fn from(value: String) -> Self {
    Self { value: napi::create_string_utf8(&value) }
  }
}

impl Into<String> for NapiString {
  fn into(self) -> String {
    self.as_string()
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiString {
  pub fn as_string(&self) -> String {
    napi::get_value_string_utf8(self.value)
  }
}
