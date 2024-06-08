use crate::napi;
use crate::types::*;
use std::cell::Cell;

#[derive(Clone, Debug)]
pub struct NapiBigint {
  value: Cell<Option<i128>>,
  handle: Option<napi::Handle>,
}

impl NapiShape for NapiBigint {}

impl NapiShapeInternal for NapiBigint {
  fn into_napi_value(self) -> napi::Handle {
    if let Some(handle) = self.handle {
      return handle
    } else {
      napi::create_bigint_words(self.value.get().unwrap())
    }
  }

  fn from_napi_value(handle: napi::Handle) -> Self {
    Self { value: Cell::new(None), handle: Some(handle) }
  }
}

// ===== I32 CONVERSION ========================================================

impl From<i32> for NapiBigint {
  fn from(value: i32) -> Self {
    Self::from(value as i128)
  }
}

// ===== U32 CONVERSION ========================================================

impl From<u32> for NapiBigint {
  fn from(value: u32) -> Self {
    Self::from(value as i128)
  }
}

// ===== I64 CONVERSION ========================================================

impl From<i64> for NapiBigint {
  fn from(value: i64) -> Self {
    Self::from(value as i128)
  }
}

// ===== U64 CONVERSION ========================================================

impl From<u64> for NapiBigint {
  fn from(value: u64) -> Self {
    Self::from(value as i128)
  }
}

// ===== I128 CONVERSION =======================================================

impl From<i128> for NapiBigint {
  fn from(value: i128) -> Self {
    Self { value: Cell::new(Some(value)), handle: None }
  }
}

impl Into<i128> for NapiBigint {
  fn into(self) -> i128 {
    match self.value.into_inner() {
      None => napi::get_value_bigint_words(self.handle.unwrap()),
      Some(value) => value,
    }
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiBigint {
  pub fn new(value: i128) -> Self {
    Self::from(value)
  }

  pub fn value(&self) -> i128 {
    self.value.get().unwrap_or_else(|| {
      let value = napi::get_value_bigint_words(self.handle.unwrap());
      self.value.set(Some(value));
      value
    })
  }
}
