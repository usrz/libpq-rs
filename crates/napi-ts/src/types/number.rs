use crate::napi;
use crate::types::*;

#[derive(Clone, Debug)]
pub struct NapiNumber {
  value: f64,
}

impl NapiShape for NapiNumber {}

impl NapiShapeInternal for NapiNumber {
  fn into_napi_value(self) -> napi::Handle {
    napi::create_double(self.value)
  }

  fn from_napi_value(handle: napi::Handle) -> Self {
    napi::expect_type_of(handle, napi::Type::napi_number);
    Self { value: napi::get_value_double(handle) }
  }
}

// ===== I32 CONVERSION ========================================================

impl From<i32> for NapiNumber {
  fn from(value: i32) -> Self {
    Self::from(value as f64)
  }
}

impl Into<i32> for NapiNumber {
  fn into(self) -> i32 {
    let value: f64 = self.into();
    return value as i32
  }
}

// ===== U32 CONVERSION ========================================================

impl From<u32> for NapiNumber {
  fn from(value: u32) -> Self {
    Self::from(value as f64)
  }
}

impl Into<u32> for NapiNumber {
  fn into(self) -> u32 {
    let value: f64 = self.into();
    return value as u32

  }
}

// ===== F64 CONVERSION ========================================================

impl From<f64> for NapiNumber {
  fn from(value: f64) -> Self {
    Self { value: value }
  }
}

impl Into<f64> for NapiNumber {
  fn into(self) -> f64 {
    self.value
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiNumber {
  pub fn new(value: f64) -> Self {
    Self::from(value)
  }

  pub fn value(&self) -> f64 {
    self.value
  }
}
