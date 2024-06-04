use crate::napi;
use crate::types::*;

#[derive(Debug)]
pub struct NapiUndefined {
  value: napi::Value,
}

impl NapiShape for NapiUndefined {}

impl NapiShapeInternal for NapiUndefined {
  fn as_napi_value(&self) -> napi::Value {
    self.value
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { value }
  }
}

impl NapiUndefined {
  pub fn new() -> Self {
    Self { value: napi::get_undefined() }
  }
}
