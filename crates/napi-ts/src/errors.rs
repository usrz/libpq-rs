use core::fmt;
use crate::napi;

// ========================================================================== //
// RESULT TYPE                                                                //
// ========================================================================== //

pub type NapiResult = Result<NapiOk, NapiErr>;

// ========================================================================== //
// "OK" TYPE => only holds the napi value pointer                             //
// ========================================================================== //

pub struct NapiOk {
  pub (crate) handle: napi::Handle,
}

impl fmt::Debug for NapiOk {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("NapiOk")
      .field(&self.handle)
      .finish()
  }
}

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
