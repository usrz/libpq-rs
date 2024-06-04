use crate::napi;
use crate::types::*;
use crate::errors::NapiResult;

#[derive(Clone,Debug)]
pub struct NapiObject {
  value: napi::Value,
}

impl NapiValue for NapiObject {}

impl NapiValueInternal for NapiObject {
  fn as_napi_value(&self) -> napi::Value {
    self.value
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { value }
  }
}

impl Into<NapiResult<NapiValues>> for NapiObject {
  fn into(self) -> NapiResult<NapiValues> {
    Ok(self.into())
  }
}

// ===== EXTRA TRAITS ==========================================================

impl NapiValueWithProperties for NapiObject {}

// ===== EXTRA METHODS =========================================================

impl NapiObject {
  pub fn new() -> Self {
    Self { value: napi::create_object() }
  }
}
