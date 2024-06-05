use core::fmt;
use std::fmt::Debug;
use crate::napi;
use std::error::Error;
use std::fmt::Display;
use std::borrow::Borrow;
use crate::NapiShape;
use crate::NapiShapeInternal;

// ========================================================================== //
// RESULT TYPE                                                                //
// ========================================================================== //

pub type NapiResult<T> = Result<T, NapiError>;

// ========================================================================== //
// RETURN VALUE                                                               //
// ========================================================================== //

#[derive(Clone, Debug)]
pub struct NapiReturn {
  value: napi::Value
}

unsafe impl Send for NapiReturn {}

impl <T: NapiShapeInternal> From<T> for NapiReturn {
  fn from(value: T) -> Self {
    Self { value: value.as_napi_value() }
  }
}

impl Into<napi::Value> for NapiReturn {
  fn into(self) -> napi::Value {
    self.value
  }
}

impl NapiReturn {
  pub fn void() -> NapiResult<NapiReturn> {
    Ok(Self { value: napi::get_undefined() })
  }
}

// ========================================================================== //
// ERROR VALUE                                                                //
// ========================================================================== //

#[derive(Debug)]
pub enum NapiError {
  Message(String),
  Error(napi::Value, napi::Reference),
}

impl Error for NapiError {}

impl Display for NapiError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // Do not coerce "to_string" here... we might not have an env setup!
    match &self {
      Self::Message(message) => write!(f, "{}", message),
      Self::Error(_, _) => write!(f, "JavaScript Error"),
    }
  }
}

impl Clone for NapiError {
  fn clone(&self) -> Self {
    match self {
      Self::Message(message) => Self::from(message.clone()),
      Self::Error(value, reference) => {
        let count = napi::reference_ref(*reference);
        println!("CLONED REF COUNT {}", count);
        Self::Error(*value, *reference)
      }
    }
  }
}

impl Drop for NapiError {
  fn drop(&mut self) {
    if let Self::Error(_, reference) = self {
      let count = napi::reference_unref(*reference);
      if count == 0 { napi::delete_reference(*reference) };
      println!("DROPPED REF COUNT {}", count);
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

impl From<napi::Value> for NapiError {
  fn from(value: napi::Value) -> Self {
    let reference = napi::create_reference(value, 1);
    println!("CREATED REF COUNT {}", 1);
    Self::Error(value, reference)
  }
}

impl Into<napi::Value> for NapiError {
  fn into(self) -> napi::Value {
    match self.borrow() {
      Self::Error(value, _) => *value,
      Self::Message(message) => napi::create_error(message.clone()),
    }
  }
}
