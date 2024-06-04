use crate::napi;
use crate::types::*;
use crate::errors::NapiResult;

#[derive(Clone,Debug)]
pub struct NapiBoolean {
  value: napi::Value,
}

impl NapiValue for NapiBoolean {}

impl NapiValueInternal for NapiBoolean {
  fn as_napi_value(&self) -> napi::Value {
    self.value
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { value }
  }
}

impl Into<NapiResult<NapiValues>> for NapiBoolean {
  fn into(self) -> NapiResult<NapiValues> {
    Ok(self.into())
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
