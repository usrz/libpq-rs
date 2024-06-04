use crate::napi;
use crate::types::*;
use crate::errors::NapiError;
use crate::errors::NapiResult;

#[derive(Debug)]
pub struct NapiString {
  pub(super) value: napi::Value,
}

impl NapiValue for NapiString {
  unsafe fn as_napi_value(&self) -> napi::Value {
    self.value
  }
}

impl Into<NapiResult<NapiValues>> for NapiString {
  fn into(self) -> NapiResult<NapiValues> {
    Ok(self.into())
  }
}

impl TryFrom<napi::Value> for NapiString {
  type Error = NapiError;

  fn try_from(value: napi::Value) -> NapiResult<Self> {
    Ok(Self { value: expect_type(value, napi::ValueType::napi_string)? })
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
