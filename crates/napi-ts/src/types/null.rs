use crate::napi;
use crate::types::*;
use crate::errors::NapiResult;

#[derive(Clone,Debug)]
pub struct NapiNull {
  value: napi::Value,
}

impl NapiValue for NapiNull {}

impl NapiValueInternal for NapiNull {
  fn as_napi_value(&self) -> napi::Value {
    self.value
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { value }
  }
}

impl Into<NapiResult<NapiValues>> for NapiNull {
  fn into(self) -> NapiResult<NapiValues> {
    Ok(self.into())
  }
}

impl NapiNull {
  pub fn new() -> Self {
    Self { value: napi::get_null() }
  }
}
