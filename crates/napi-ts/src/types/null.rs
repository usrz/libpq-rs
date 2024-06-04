use crate::napi;
use crate::types::*;
use crate::errors::NapiError;
use crate::errors::NapiResult;

#[derive(Debug)]
pub struct NapiNull {
  pub(super) value: napi::Value,
}

impl NapiValue for NapiNull {
  unsafe fn as_napi_value(&self) -> napi::Value {
    self.value
  }
}

impl Into<NapiResult<NapiValues>> for NapiNull {
  fn into(self) -> NapiResult<NapiValues> {
    Ok(self.into())
  }
}

impl TryFrom<napi::Value> for NapiNull {
  type Error = NapiError;

  fn try_from(value: napi::Value) -> NapiResult<Self> {
    Ok(Self { value: expect_type(value, napi::ValueType::napi_null)? })
  }
}

impl NapiNull {
  pub fn new() -> Self {
    Self { value: napi::get_null() }
  }
}
