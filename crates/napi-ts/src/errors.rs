use core::fmt;
use std::fmt::Debug;
use crate::napi;
use std::error::Error;
use std::fmt::Display;
// use crate::NapiValue;

// ========================================================================== //
// RESULT TYPE                                                                //
// ========================================================================== //

pub type NapiResult<T> = Result<T, NapiError>;

// // ========================================================================== //
// // RETURN VALUE                                                               //
// // ========================================================================== //

// #[derive(Clone, Debug)]
// pub struct NapiReturn {
//   value: NapiValue
// }

// impl <T: NapiShape> From<T> for NapiReturn {
//   fn from(value: T) -> Self {
//     Self { value: value.into() }
//   }
// }

// impl From<napi::Handle> for NapiReturn {
//   fn from(value: napi::Handle) -> Self {
//     Self { value: value.into() }
//   }
// }

// impl Into<napi::Handle> for NapiReturn {
//   fn into(self) -> napi::Handle {
//     self.value.into()
//   }
// }

// impl NapiReturn {
//   pub fn void() -> NapiResult<NapiReturn> {
//     Ok(Self { value: napi::get_undefined().into() })
//   }
// }

// ========================================================================== //
// ERROR VALUE                                                                //
// ========================================================================== //

#[derive(Debug)]
pub struct NapiError {
  message: String,
}

impl Error for NapiError {}

impl Display for NapiError {
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
    Self { message: message.to_string(), }
  }
}

// impl Into<napi::Handle> for NapiError {
//   fn into(self) -> napi::Handle {
//     napi::create_error(self.message)
//   }
// }
