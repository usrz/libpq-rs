use crate::napi;
use crate::types::*;
use crate::errors::NapiError;
use crate::errors::NapiResult;

pub struct NapiBoolean {
  pub(super) value: napi::Value,
}

impl NapiValue for NapiBoolean {
  unsafe fn as_napi_value(&self) -> napi::Value {
    self.value
  }
}

impl Into<NapiResult<NapiValues>> for NapiBoolean {
  fn into(self) -> NapiResult<NapiValues> {
    Ok(self.into())
  }
}

impl TryFrom<napi::Value> for NapiBoolean {
  type Error = NapiError;

  fn try_from(value: napi::Value) -> NapiResult<Self> {
    Ok(Self { value: expect_type(value, napi::ValueType::napi_boolean)? })
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
