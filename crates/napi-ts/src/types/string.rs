use crate::napi;
use crate::types::*;

#[derive(Clone, Debug)]
pub struct NapiString {
  value: String,
}

impl NapiShape for NapiString {}

impl NapiShapeInternal for NapiString {
  fn into_napi_value(self) -> napi::Handle {
    napi::create_string_utf8(self.value.as_str())
  }

  fn from_napi_value(handle: napi::Handle) -> Self {
    napi::expect_type_of(handle, napi::Type::napi_string);
    Self { value: napi::get_value_string_utf8(handle) }
  }
}

// ===== &STR CONVERSION =======================================================

impl <S: AsRef<str>> From<S> for NapiString {
  fn from(value: S) -> Self {
    Self { value: value.as_ref().to_string() }
  }
}

// ===== STRING CONVERSION =====================================================

impl Into<String> for NapiString {
  fn into(self) -> String {
    self.value
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiString {
  pub fn new<S: AsRef<str>>(value: S) -> Self {
    Self::from(value)
  }

  pub fn value(&self) -> String {
    self.value.clone()
  }
}
