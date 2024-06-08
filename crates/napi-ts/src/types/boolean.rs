use crate::napi;
use crate::types::*;

#[derive(Clone, Debug)]
pub struct NapiBoolean {
  value: bool,
}

impl NapiShape for NapiBoolean {}

impl NapiShapeInternal for NapiBoolean {
  fn into_napi_value(self) -> napi::Handle {
    napi::get_boolean(self.value)
  }

  fn from_napi_value(handle: napi::Handle) -> Self {
    napi::expect_type_of(handle, napi::Type::napi_boolean);
    Self { value: napi::get_value_bool(handle) }
  }
}

// ===== BOOL CONVERSION =======================================================

impl From<bool> for NapiBoolean {
  fn from(value: bool) -> Self {
    Self { value }
  }
}

impl Into<bool> for NapiBoolean {
  fn into(self) -> bool {
    self.value
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiBoolean {
  pub fn new(value: bool) -> Self {
    Self::from(value)
  }

  pub fn value(&self) -> bool {
    self.value
  }
}
