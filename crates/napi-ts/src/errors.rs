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
pub enum NapiError {
  Message(String),
  Error(NapiValue),
}

impl Error for NapiError {}

impl Display for NapiError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // Do not coerce "to_string" here... we might not have an env setup!
    match &self {
      Self::Message(message) => write!(f, "{}", message),
      Self::Error(value) => write!(f, "JavaScript Error: {:?}", value),
    }
  }
}

impl From<String> for NapiError {
  fn from(message: String) -> Self {
    Self::Message(message)
  }
}

impl From<&str> for NapiError {
  fn from(message: &str) -> Self {
    Self::Message(message.to_string())
  }
}

impl <T: NapiShape> From<T> for NapiError {
  fn from(value: T) -> Self {
    Self::Error(value.into())
  }
}

impl From<napi::Value> for NapiError {
  fn from(value: napi::Value) -> Self {
    Self::Error(value.into())
  }
}

impl Into<napi::Value> for NapiError {
  fn into(self) -> napi::Value {
    match self {
      Self::Message(message) => napi::create_error(message.clone()),
      Self::Error(value) => value.into(),
    }
  }
}
