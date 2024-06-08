use crate::napi;
use crate::types::*;

#[derive(Clone, Debug)]
pub struct NapiNull {
  handle: Option<napi::Handle>
}

impl NapiShape for NapiNull {}

impl NapiShapeInternal for NapiNull {
  fn into_napi_value(self) -> napi::Handle {
    match self.handle {
      Some(handle) => handle,
      None => napi::get_null(),
    }
  }

  fn from_napi_value(handle: napi::Handle) -> Self {
    Self { handle: Some(handle) }
  }
}

impl NapiNull {
  pub fn new() -> Self {
    Self { handle: None }
  }
}
