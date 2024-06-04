use crate::napi;
use crate::types::*;

#[derive(Clone,Debug)]
pub struct NapiUndefined {
  value: napi::Value,
}

impl NapiValue for NapiUndefined {}

impl NapiValueInternal for NapiUndefined {
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
