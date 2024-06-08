use crate::napi;
use crate::types::*;

#[derive(Clone, Debug)]
pub struct NapiUndefined {
  handle: Option<napi::Handle>
}

impl NapiShape for NapiUndefined {}

impl NapiShapeInternal for NapiUndefined {
  fn into_napi_value(self) -> napi::Handle {
    match self.handle {
      Some(handle) => handle,
      None => napi::get_undefined(),
    }
  }

  fn from_napi_value(handle: napi::Handle) -> Self {
    Self { handle: Some(handle) }
  }
}

impl NapiUndefined {
  pub fn new() -> Self {
    Self { handle: None }
  }
}
