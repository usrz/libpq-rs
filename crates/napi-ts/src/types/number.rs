use crate::napi;
use crate::errors::*;
use crate::types::*;

pub struct NapiNumber {
  pub(super) value: napi::Value,
}

impl NapiValue for NapiNumber {
  unsafe fn as_napi_value(&self) -> napi::Value {
    self.value
  }
}

impl Into<NapiResult<NapiValues>> for NapiNumber {
  fn into(self) -> NapiResult<NapiValues> {
    Ok(self.into())
  }
}

impl TryFrom<napi::Value> for NapiNumber {
  type Error = NapiError;

  fn try_from(value: napi::Value) -> NapiResult<Self> {
    Ok(Self { value: expect_type(value, napi::ValueType::napi_string)? })
  }
}

// ===== I32 CONVERSION ========================================================

impl From<i32> for NapiNumber {
  fn from(value: i32) -> Self {
    Self { value: napi::create_int32(value) }
  }
}

impl Into<i32> for NapiNumber {
  fn into(self) -> i32 {
    self.as_i32()
  }
}

// ===== U32 CONVERSION ========================================================

impl From<u32> for NapiNumber {
  fn from(value: u32) -> Self {
    Self { value: napi::create_uint32(value) }
  }
}

impl Into<u32> for NapiNumber {
  fn into(self) -> u32 {
    self.as_u32()
  }
}

// ===== I64 CONVERSION ========================================================

impl From<i64> for NapiNumber {
  fn from(value: i64) -> Self {
    Self { value: napi::create_int64(value) }
  }
}

impl Into<i64> for NapiNumber {
  fn into(self) -> i64 {
    self.as_i64()
  }
}

// ===== F64 CONVERSION ========================================================

impl From<f64> for NapiNumber {
  fn from(value: f64) -> Self {
    Self { value: napi::create_double(value) }
  }
}

impl Into<f64> for NapiNumber {
  fn into(self) -> f64 {
    self.as_f64()
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiNumber {
  pub fn as_i32(&self) -> i32 {
    napi::get_value_int32(self.value)
  }

  pub fn as_u32(&self) -> u32 {
    napi::get_value_uint32(self.value)
  }

  pub fn as_i64(&self) -> i64 {
    napi::get_value_int64(self.value)
  }

  pub fn as_f64(&self) -> f64 {
    napi::get_value_double(self.value)
  }
}