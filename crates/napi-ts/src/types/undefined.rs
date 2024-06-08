use crate::napi;
use crate::types::*;

#[derive(Clone, Debug)]
pub struct NapiUndefined {}

impl NapiShape for NapiUndefined {}

impl NapiShapeInternal for NapiUndefined {
  fn into_napi_value(self) -> napi::Handle {
    napi::get_undefined()
  }

  fn from_napi_value(_value: napi::Handle) -> Self {
    Self {}
  }
}

impl NapiUndefined {
  pub fn new() -> Self {
    Self {}
  }
}
