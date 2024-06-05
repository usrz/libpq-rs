use crate::napi;
use crate::types::*;

#[derive(Clone,Debug)]
pub struct NapiObject {
  reference: NapiReference,
}

impl NapiShape for NapiObject {}

impl NapiShapeInternal for NapiObject {
  fn into_napi_value(self) -> napi::Value {
    self.reference.value()
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { reference: value.into() }
  }
}

// ===== EXTRA TRAITS ==========================================================

impl NapiValueWithProperties for NapiObject {}

// ===== EXTRA METHODS =========================================================

impl NapiObject {
  pub fn new() -> Self {
    Self::from_napi_value(napi::create_object())
  }
}
