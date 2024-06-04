use core::fmt;
use std::fmt::Debug;
use crate::napi;
use crate::NapiShape;

/* ===== PUBLIC: NapiResult ================================================= */

pub type NapiResult<T> = Result<T, NapiError>;

/* ===== PUBLIC: NapiError ================================================== */

#[derive(Debug)]
pub struct NapiError {
  message: Option<String>,
  error: Option<napi::Value>,
}

impl std::fmt::Display for NapiError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(message) = &self.message {
      write!(f, "{}", message)
    } else if let Some(error) = self.error {
      write!(f, "JavaScript Error ({:?})", error)
    } else {
      write!(f, "Unknown Error")
    }
  }
}

impl From<String> for NapiError {
  fn from(message: String) -> Self {
    Self { message: Some(message), error: None }
  }
}

impl From<&str> for NapiError {
  fn from(message: &str) -> Self {
    Self { message: Some(message.to_string()), error: None }
  }
}

impl <T: NapiShape> From<T> for NapiError {
  fn from(value: T) -> Self {
    value.as_napi_value().into()
  }
}

impl From<napi::Value> for NapiError {
  fn from(value: napi::Value) -> Self {
    // todo: increase reference count
    Self { message: None, error: Some(value) }
  }
}

impl Into<napi::Value> for NapiError {
  fn into(self) -> napi::Value {
    if let Some(error) = self.error {
      // decrease reference count!!!
      return error
    }

    let message = match self.message {
      None => "Unknown Error".to_string(),
      Some(message) => message,
    };

    napi::create_error(message)
  }
}
