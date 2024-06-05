use crate::napi;
use crate::types::*;

#[derive(Clone, Debug)]
pub struct NapiString {
  value: String,
}

impl NapiShape for NapiString {}

impl NapiShapeInternal for NapiString {
  fn into_napi_value(self) -> napi::Value {
    napi::create_string_utf8(&self.value)
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { value: napi::get_value_string_utf8(value) }
  }
}

// ===== &STR CONVERSION =======================================================

impl From<&str> for NapiString {
  fn from(value: &str) -> Self {
    Self { value: value.to_string() }
  }
}

// ===== STRING CONVERSION =====================================================

impl From<String> for NapiString {
  fn from(value: String) -> Self {
    Self { value }
  }
}

impl Into<String> for NapiString {
  fn into(self) -> String {
    self.value
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiString {
  pub fn value(&self) -> String {
    self.value.clone()
  }
}
