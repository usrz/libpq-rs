use core::fmt;
use crate::napi;
use crate::*;

// ========================================================================== //
// RESULT TYPE                                                                //
// ========================================================================== //

pub type NapiResult<'a, T> = Result<NapiRef<'a, T>, NapiErr>;

// ========================================================================== //
// "ERR" TYPE => holds an error message and an optional the napi value ptr    //
// ========================================================================== //

pub struct NapiErr {
  pub (crate) message: String,
  pub (crate) handle: Option<napi::Handle>,
}

impl fmt::Debug for NapiErr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut f = f.debug_struct("NapiErr");
    match self.handle {
      Some(value) => f.field("@", &value),
      None => f.field("message", &self.message),
    }.finish()
  }
}

impl <T: AsRef<str>> From<T> for NapiErr {
  fn from(value: T) -> Self {
    Self {
      message: value.as_ref().to_string(),
      handle: None,
    }
  }
}
