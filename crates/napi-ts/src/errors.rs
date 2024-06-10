use core::fmt;
use std::error::Error;
use crate::napi;
use crate::NapiType;

// ========================================================================== //
// "OK" TYPE => only holds the napi value pointer                             //
// ========================================================================== //

pub struct NapiOk {
  pub (crate) value: napi::Handle,
}

pub struct NapiErr {
  pub (crate) message: String,
  pub (crate) value: Option<napi::Handle>,
}

impl fmt::Debug for NapiErr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut f = f.debug_struct("NapiErr");
    match self.value {
      Some(value) => f.field("@", &value),
      None => f.field("message", &self.message),
    }.finish()
  }
}

impl fmt::Display for NapiErr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(&self.message)
  }
}

impl Error for NapiErr {}

impl From<&str> for NapiErr {
  fn from(value: &str) -> Self {
    Self { message: value.to_string(), value: None }
  }
}

impl From<String> for NapiErr {
  fn from(value: String) -> Self {
    Self { message: value.clone(), value: None }
  }
}

// ========================================================================== //
// RESULT TYPE                                                                //
// ========================================================================== //

pub type NapiResult = Result<NapiOk, NapiErr>;

// ===== OK ================================================================= //

pub trait NapiIntoOk {
  fn ok(self) -> NapiResult;
}

impl <'a, T: NapiType<'a>> NapiIntoOk for T {
  fn ok(self) -> NapiResult {
    Ok(NapiOk { value: self.get_napi_handle().handle })
  }
}

// ===== ERR ================================================================ //

pub trait NapiIntoErr {
  fn into_err(self) -> NapiResult;
}

impl <'a, T: NapiType<'a>> NapiIntoErr for T {
  fn into_err(self) -> NapiResult {
    Err(NapiErr {
      message: "JavaScript Error".to_string(),
      value: Some(self.get_napi_handle().handle),
    })
  }
}

impl NapiIntoErr for &str {
  fn into_err(self) -> NapiResult {
    Err(NapiErr {
      message: self.to_string(),
      value: None,
    })
  }
}

impl NapiIntoErr for String {
  fn into_err(self) -> NapiResult {
    Err(NapiErr {
      message: self.clone(),
      value: None,
    })
  }
}
