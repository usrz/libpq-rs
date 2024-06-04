use crate::napi;
use crate::types::*;

#[derive(Debug)]
pub struct NapiNull {
  value: napi::Value,
}

impl NapiShape for NapiNull {}

impl NapiShapeInternal for NapiNull {
  fn as_napi_value(&self) -> napi::Value {
    self.value
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { value }
  }
}

impl NapiNull {
  pub fn new() -> Self {
    Self { value: napi::get_null() }
  }
}
