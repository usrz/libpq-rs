use crate::napi;
use crate::types::*;
use crate::errors::NapiResult;
use crate::errors::NapiError;

pub struct NapiUndefined {
  pub(super) value: napi::Value,
}

impl NapiValue for NapiUndefined {
  unsafe fn as_napi_value(&self) -> napi::Value {
    self.value
  }
}

impl Into<NapiResult<NapiValues>> for NapiUndefined {
  fn into(self) -> NapiResult<NapiValues> {
    Ok(self.into())
  }
}

impl TryFrom<napi::Value> for NapiUndefined {
  type Error = NapiError;

  fn try_from(value: napi::Value) -> NapiResult<Self> {
    Ok(Self { value: expect_type(value, napi::ValueType::napi_undefined)? })
  }
}

impl NapiUndefined {
  pub fn new() -> Self {
    Self { value: napi::get_undefined() }
  }
}
