use crate::napi;
use crate::types::*;

#[derive(Clone, Debug)]
pub struct NapiBigint {
  value: i128,
}

impl NapiShape for NapiBigint {}

impl NapiShapeInternal for NapiBigint {
  fn as_napi_value(&self) -> napi::Value {
    napi::create_bigint_words(self.value)
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { value: napi::get_value_bigint_words(value) }
  }
}

// ===== I32 CONVERSION ========================================================

impl From<i32> for NapiBigint {
  fn from(value: i32) -> Self {
    Self { value: value as i128 }
  }
}

impl Into<i32> for NapiBigint {
  fn into(self) -> i32 {
    self.value as i32
  }
}

// ===== U32 CONVERSION ========================================================

impl From<u32> for NapiBigint {
  fn from(value: u32) -> Self {
    Self { value: value as i128 }
  }
}

impl Into<u32> for NapiBigint {
  fn into(self) -> u32 {
    self.value as u32
  }
}

// ===== I64 CONVERSION ========================================================

impl From<i64> for NapiBigint {
  fn from(value: i64) -> Self {
    Self { value: value as i128 }
  }
}

impl Into<i64> for NapiBigint {
  fn into(self) -> i64 {
    self.value as i64
  }
}

// ===== U64 CONVERSION ========================================================

impl From<u64> for NapiBigint {
  fn from(value: u64) -> Self {
    Self { value: value as i128 }
  }
}

impl Into<u64> for NapiBigint {
  fn into(self) -> u64 {
    self.value as u64
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
  pub fn value(&self) -> i128 {
    self.value
  }
}
