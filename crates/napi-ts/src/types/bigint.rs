use crate::napi;
use crate::types::*;
use crate::errors::NapiError;
use crate::errors::NapiResult;

#[derive(Clone,Debug)]
pub struct NapiBigint {
  value: napi::Value,
}

impl NapiValue for NapiBigint {}

impl NapiValueInternal for NapiBigint {
  fn as_napi_value(&self) -> napi::Value {
    self.value
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { value }
  }
}

impl Into<NapiResult<NapiValues>> for NapiBigint {
  fn into(self) -> NapiResult<NapiValues> {
    Ok(self.into())
  }
}

// ===== I32 CONVERSION ========================================================

impl From<i32> for NapiBigint {
  fn from(value: i32) -> Self {
    Self::from(value as i64)
  }
}

impl TryInto<i32> for NapiBigint {
  type Error = NapiError;

  fn try_into(self) -> NapiResult<i32> {
    self.try_as_i32()
  }
}

// ===== U32 CONVERSION ========================================================

impl From<u32> for NapiBigint {
  fn from(value: u32) -> Self {
    Self::from(value as i64)
  }
}

impl TryInto<u32> for NapiBigint {
  type Error = NapiError;

  fn try_into(self) -> NapiResult<u32> {
    self.try_as_u32()
  }
}

// ===== I64 CONVERSION ========================================================

impl From<i64> for NapiBigint {
  fn from(value: i64) -> Self {
    Self { value: napi::create_bigint_int64(value) }
  }
}

impl TryInto<i64> for NapiBigint {
  type Error = NapiError;

  fn try_into(self) -> NapiResult<i64> {
    self.try_as_i64()
  }
}

// ===== U64 CONVERSION ========================================================

impl From<u64> for NapiBigint {
  fn from(value: u64) -> Self {
    Self { value: napi::create_bigint_uint64(value) }
  }
}

impl TryInto<u64> for NapiBigint {
  type Error = NapiError;

  fn try_into(self) -> NapiResult<u64> {
    self.try_as_u64()
  }
}

// ===== I128 CONVERSION =======================================================

impl From<i128> for NapiBigint {
  fn from(value: i128) -> Self {
    Self { value: napi::create_bigint_words_i128(value) }
  }
}

impl TryInto<i128> for NapiBigint {
  type Error = NapiError;

  fn try_into(self) -> NapiResult<i128> {
    self.try_as_i128()
  }
}

// ===== U128 CONVERSION =======================================================

impl From<u128> for NapiBigint {
  fn from(value: u128) -> Self {
    Self { value: napi::create_bigint_words_u128(value) }
  }
}

impl TryInto<u128> for NapiBigint {
  type Error = NapiError;

  fn try_into(self) -> NapiResult<u128> {
    self.try_as_u128()
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiBigint {
  pub fn try_as_i32(&self) -> NapiResult<i32> {
    let value: i64 = self.try_as_i64()?;

    i32::try_from(value)
      .map_err(|_| "Unable to convert JavaScript \"bigint\" to Rust \"i32\"".into())
  }

  pub fn try_as_u32(&self) -> NapiResult<u32> {
    let value: u64 = self.try_as_u64()?;

    u32::try_from(value)
      .map_err(|_| "Unable to convert JavaScript \"bigint\" to Rust \"u32\"".into())
  }

  pub fn try_as_i64(&self) -> NapiResult<i64> {
    napi::get_value_bigint_int64(self.value)
  }

  pub fn try_as_u64(&self) -> NapiResult<u64> {
    napi::get_value_bigint_uint64(self.value)
  }

  pub fn try_as_i128(&self) -> NapiResult<i128> {
    napi::get_value_bigint_words_i128(self.value)
  }

  pub fn try_as_u128(&self) -> NapiResult<u128> {
    napi::get_value_bigint_words_u128(self.value)
  }
}
