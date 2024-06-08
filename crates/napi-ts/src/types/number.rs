use crate::napi;
use crate::types::*;
use std::cell::Cell;

#[derive(Clone, Debug)]
pub struct NapiNumber {
  value: Cell<Option<f64>>,
  handle: Option<napi::Handle>,
}

impl NapiShape for NapiNumber {}

impl NapiShapeInternal for NapiNumber {
  fn into_napi_value(self) -> napi::Handle {
    match self.handle {
      None => napi::create_double(self.value.get().unwrap()),
      Some(handle) => handle,
    }
  }

  fn from_napi_value(handle: napi::Handle) -> Self {
    Self { handle: Some(handle), value: Cell::new(None) }
  }
}

// ===== I32 CONVERSION ========================================================

impl From<i32> for NapiNumber {
  fn from(value: i32) -> Self {
    Self::from(value as f64)
  }
}

impl Into<i32> for NapiNumber {
  fn into(self) -> i32 {
    let value: f64 = self.into();
    return value as i32
  }
}

// ===== U32 CONVERSION ========================================================

impl From<u32> for NapiNumber {
  fn from(value: u32) -> Self {
    Self::from(value as f64)
  }
}

impl Into<u32> for NapiNumber {
  fn into(self) -> u32 {
    let value: f64 = self.into();
    return value as u32

  }
}

// ===== F64 CONVERSION ========================================================

impl From<f64> for NapiNumber {
  fn from(value: f64) -> Self {
    Self { value: Cell::new(Some(value)), handle: None }
  }
}

impl Into<f64> for NapiNumber {
  fn into(self) -> f64 {
    match self.value.into_inner() {
      None => napi::get_value_double(self.handle.unwrap()),
      Some(value) => value,
    }
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiNumber {
  pub fn new(value: f64) -> Self {
    Self::from(value)
  }

  pub fn value(&self) -> f64 {
    self.value.get().unwrap_or_else(|| {
      let value = napi::get_value_double(self.handle.unwrap());
      self.value.set(Some(value));
      value
    })
  }
}
