use crate::napi;
use crate::types::*;

#[derive(Clone,Debug)]
pub struct NapiObject {
  value: napi::Value,
}

impl NapiShape for NapiObject {}

impl NapiShapeInternal for NapiObject {
  fn as_napi_value(&self) -> napi::Value {
    self.value
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { value }
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
