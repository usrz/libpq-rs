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
  pub (crate) value: nodejs_sys::napi_value,
}

impl fmt::Debug for NapiOk {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("NapiOk")
      .field("@", &self.value)
      .finish()
  }
}

impl From<napi::Handle<'_>> for NapiOk {
  fn from(handle: napi::Handle) -> Self {
    Self { value: handle.value() }
  }
}

// ========================================================================== //
// "ERR" TYPE => holds an error message and an optional the napi value ptr    //
// ========================================================================== //

pub struct NapiErr {
  pub message: String,
  pub value: Option<nodejs_sys::napi_value>,
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

impl <T: AsRef<str>> From<T> for NapiErr {
  fn from(value: T) -> Self {
    Self {
      message: value.as_ref().to_string(),
      value: None,
    }
  }
}

impl From<napi::Handle<'_>> for NapiErr {
  fn from(handle: napi::Handle) -> Self {
    Self {
      message: "JavaScript Error".to_string(),
      value: Some(handle.value()),
    }
  }
}

impl NapiErr {
  pub (crate) fn throw(&self, env: napi::Env) {
    env.handle(self.value.unwrap_or_else(|| {
      env.create_error(&self.message).value()
    })).throw();
  }
}
