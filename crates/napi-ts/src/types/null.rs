use crate::napi;
use crate::types::*;

#[derive(Clone, Debug)]
pub struct NapiNull {}

impl NapiShape for NapiNull {}

impl NapiShapeInternal for NapiNull {
  fn into_napi_value(self) -> napi::Handle {
    napi::get_null()
  }

  fn from_napi_value(handle: napi::Handle) -> Self {
    napi::expect_type_of(handle, napi::Type::napi_null);
    Self {}
  }
}

impl NapiNull {
  pub fn new() -> Self {
    Self {}
  }
}
