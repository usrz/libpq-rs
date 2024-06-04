use core::fmt;
use std::fmt::Debug;

/* ===== PUBLIC: NapiResult ================================================= */

pub type NapiResult<T> = Result<T, NapiError>;

/* ===== PUBLIC: NapiError ================================================== */

#[derive(Debug)]
pub struct NapiError {
  message: String,
}

impl std::fmt::Display for NapiError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl From<String> for NapiError {
  fn from(message: String) -> Self {
    Self { message }
  }
}

impl From<&str> for NapiError {
  fn from(message: &str) -> Self {
    Self { message: message.to_string() }
  }
}
