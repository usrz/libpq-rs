use crate::napi;
use crate::types::*;

#[derive(Clone, Debug)]
pub struct NapiBigint {
  value: i128,
}

impl NapiShape for NapiBigint {}

impl NapiShapeInternal for NapiBigint {
  fn into_napi_value(self) -> napi::Handle {
    napi::create_bigint_words(self.value)
  }

  fn from_napi_value(handle: napi::Handle) -> Self {
    napi::expect_type_of(handle, napi::Type::napi_bigint);
    Self { value: napi::get_value_bigint_words(handle) }
  }
}

// ===== I32 CONVERSION ========================================================

impl From<i32> for NapiBigint {
  fn from(value: i32) -> Self {
    Self::from(value as i128)
  }
}

// ===== U32 CONVERSION ========================================================

impl From<u32> for NapiBigint {
  fn from(value: u32) -> Self {
    Self::from(value as i128)
  }
}

// ===== I64 CONVERSION ========================================================

impl From<i64> for NapiBigint {
  fn from(value: i64) -> Self {
    Self::from(value as i128)
  }
}

// ===== U64 CONVERSION ========================================================

impl From<u64> for NapiBigint {
  fn from(value: u64) -> Self {
    Self::from(value as i128)
  }
}

// ===== I128 CONVERSION =======================================================

impl From<i128> for NapiBigint {
  fn from(value: i128) -> Self {
    Self { value }
  }
}

impl Into<i128> for NapiBigint {
  fn into(self) -> i128 {
    self.value
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiBigint {
  pub fn new(value: i128) -> Self {
    Self::from(value)
  }

  pub fn value(&self) -> i128 {
    self.value
  }
}
