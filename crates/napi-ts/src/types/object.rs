use crate::napi;
use crate::types::*;

#[derive(Clone)]
pub struct NapiObject {
  reference: NapiReference,
}

impl Debug for NapiObject {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiExternal")
      .field("@", &self.reference.handle())
      .finish()
  }
}

impl NapiShape for NapiObject {}

impl NapiShapeInternal for NapiObject {
  fn into_napi_value(self) -> napi::Handle {
    self.reference.handle()
  }

  fn from_napi_value(handle: napi::Handle) -> Self {
    Self { reference: handle.into() }
  }
}

// ===== EXTRA TRAITS ==========================================================

impl NapiValueWithProperties for NapiObject {}

// ===== EXTRA METHODS =========================================================

impl NapiObject {
  pub fn new() -> Self {
    Self::from_napi_value(napi::create_object())
  }
}
