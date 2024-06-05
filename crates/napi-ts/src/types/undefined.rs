use crate::napi;
use crate::types::*;

#[derive(Clone, Debug)]
pub struct NapiUndefined {}

impl NapiShape for NapiUndefined {}

impl NapiShapeInternal for NapiUndefined {
  fn as_napi_value(self) -> napi::Value {
    napi::get_undefined()
  }

  fn from_napi_value(_value: napi::Value) -> Self {
    Self {} // TODO: rethink
  }
}

impl NapiUndefined {
  pub fn new() -> Self {
    Self {}
  }
}
