use core::fmt;
use std::fmt::Debug;
use crate::napi;
use std::error::Error;
use std::fmt::Display;
use crate::NapiShape;
use crate::NapiValue;

// ========================================================================== //
// RESULT TYPE                                                                //
// ========================================================================== //

pub type NapiResult<T> = Result<T, NapiError>;

// ========================================================================== //
// RETURN VALUE                                                               //
// ========================================================================== //

#[derive(Clone, Debug)]
pub struct NapiReturn {
  value: NapiValue
}

impl <T: NapiShape> From<T> for NapiReturn {
  fn from(value: T) -> Self {
    Self { value: value.into() }
  }
}

impl From<napi::Value> for NapiReturn {
  fn from(value: napi::Value) -> Self {
    Self { value: value.into() }
  }
}

impl Into<napi::Value> for NapiReturn {
  fn into(self) -> napi::Value {
    self.value.into()
  }
}

impl NapiReturn {
  pub fn void() -> NapiResult<NapiReturn> {
    Ok(Self { value: napi::get_undefined().into() })
  }
}

// ========================================================================== //
// ERROR VALUE                                                                //
// ========================================================================== //

#[derive(Debug)]
pub struct NapiError {
  message: String,
  error: Option<NapiValue>,
}

impl Error for NapiError {}

impl Display for NapiError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl From<String> for NapiError {
  fn from(message: String) -> Self {
    Self { message, error: None }
  }
}

impl From<&str> for NapiError {
  fn from(message: &str) -> Self {
    Self { message: message.to_string(), error: None }
  }
}

impl <T: NapiShape> From<T> for NapiError {
  fn from(value: T) -> Self {
    let value: NapiValue = value.into();
    value.into()
  }
}

impl From<NapiValue> for NapiError {
  fn from(value: NapiValue) -> Self {
    Self { message: format!("JavaScript Error: {:?}", value), error: Some(value) }
  }
}

impl Into<napi::Value> for NapiError {
  fn into(self) -> napi::Value {
    if let Some(error) = self.error {
      return error.into()
    } else {
      napi::create_error(self.message)
    }
  }
}
