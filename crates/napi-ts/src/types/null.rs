use crate::napi;
use crate::types::*;

#[derive(Clone, Debug)]
pub struct NapiNull {}

impl NapiShape for NapiNull {}

impl NapiShapeInternal for NapiNull {
  fn as_napi_value(&self) -> napi::Value {
    napi::get_null()
  }

  fn from_napi_value(_: napi::Value) -> Self {
    Self {} // TODO: rethink...
  }
}

impl NapiNull {
  pub fn new() -> Self {
    Self {}
  }
}
