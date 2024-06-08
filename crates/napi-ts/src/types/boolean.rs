use crate::napi;
use crate::types::*;
use std::cell::Cell;

#[derive(Clone, Debug)]
pub struct NapiBoolean {
  value: Cell<Option<bool>>,
  handle: Option<napi::Handle>,
}

impl NapiShape for NapiBoolean {}

impl NapiShapeInternal for NapiBoolean {
  fn into_napi_value(self) -> napi::Handle {
    if let Some(handle) = self.handle {
      return handle
    } else {
      napi::get_boolean(self.value.get().unwrap())
    }
  }

  fn from_napi_value(handle: napi::Handle) -> Self {
    Self { value: Cell::new(None), handle: Some(handle) }
  }
}

// ===== BOOL CONVERSION =======================================================

impl From<bool> for NapiBoolean {
  fn from(value: bool) -> Self {
    Self { value: Cell::new(Some(value)), handle: None }
  }
}

impl Into<bool> for NapiBoolean {
  fn into(self) -> bool {
    match self.value.into_inner() {
      None => napi::get_value_bool(self.handle.unwrap()),
      Some(value) => value,
    }
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiBoolean {
  pub fn value(&self) -> bool {
    self.value.get().unwrap_or_else(|| {
      let value = napi::get_value_bool(self.handle.unwrap());
      self.value.set(Some(value));
      value
    })
  }
}
