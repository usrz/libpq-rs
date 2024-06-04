use crate::napi;
use crate::types::*;

#[derive(Clone,Debug)]
pub struct NapiBoolean {
  value: napi::Value,
}

impl NapiShape for NapiBoolean {}

impl NapiShapeInternal for NapiBoolean {
  fn as_napi_value(&self) -> napi::Value {
    self.value
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { value }
  }
}

// ===== BOOL CONVERSION =======================================================

impl From<bool> for NapiBoolean {
  fn from(value: bool) -> Self {
    Self { value: napi::get_boolean(value) }
  }
}

impl Into<bool> for NapiBoolean {
  fn into(self) -> bool {
    self.as_bool()
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiBoolean {
  pub fn as_bool(&self) -> bool {
    napi::get_value_bool(self.value)
  }
}
