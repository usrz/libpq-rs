use crate::napi;
use crate::types::*;
use crate::errors::NapiError;
use crate::errors::NapiResult;

#[derive(Debug)]
pub struct NapiObject {
  pub(super) value: napi::Value,
}

impl NapiValue for NapiObject {
  unsafe fn as_napi_value(&self) -> napi::Value {
    self.value
  }
}

impl Into<NapiResult<NapiValues>> for NapiObject {
  fn into(self) -> NapiResult<NapiValues> {
    Ok(self.into())
  }
}

impl TryFrom<napi::Value> for NapiObject {
  type Error = NapiError;

  fn try_from(value: napi::Value) -> NapiResult<Self> {
    Ok(Self { value: expect_type(value, napi::ValueType::napi_object)? })
  }
}

impl ToOwned for NapiObject {
  type Owned = NapiObject;

  fn to_owned(&self) -> NapiObject {
    Self { value: self.value }
  }
}

// ===== EXTRA TRAITS ==========================================================

impl NapiValueWithProperties for NapiObject {}

// ===== EXTRA METHODS =========================================================

impl NapiObject {
  pub fn new() -> Self {
    Self { value: napi::create_object() }
  }
}
