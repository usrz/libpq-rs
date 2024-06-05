use crate::napi;
use crate::types::*;

#[derive(Debug)]
pub struct NapiObject {
  value: napi::Value,
  reference: napi::Reference,
}

impl NapiShape for NapiObject {}

impl Clone for NapiObject {
  fn clone(&self) -> Self {
    napi::reference_ref(self.reference);
    Self { value: self.value, reference: self.reference }
  }
}

impl Drop for NapiObject {
  fn drop(&mut self) {
    napi::reference_unref(self.reference);
  }
}

impl NapiShapeInternal for NapiObject {
  fn as_napi_value(self) -> napi::Value {
    self.value
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { value, reference: napi::create_reference(value, 1) }
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
