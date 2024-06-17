use crate::NapiRef;
use crate::NapiTypeOf;
use crate::napi;
use std::fmt;

// ========================================================================== //
// RESULT TYPE                                                                //
// ========================================================================== //

pub type NapiResult<'b, R> = Result<NapiRef<'b, R>, NapiErr>;

// ========================================================================== //
// "ERR" TYPE => holds an error message and an optional the napi value ptr    //
// ========================================================================== //

pub struct NapiErr {
  message: String,
  handle: Option<napi::Handle>,
}

impl fmt::Debug for NapiErr {
  fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut debug = fm.debug_tuple("NapiErr");
    match self.handle {
      Some(handle) => debug.field(&handle).field(&self.message),
      None => debug.field(&self.message)
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

impl NapiErr {
  pub (crate) fn from_handle(handle: napi::Handle) -> Self {
    let message = match handle.is_error() {
      false => handle.coerce_to_string(),
      true => {
        let message = handle.get_named_property("message");
        match message.type_of() {
          NapiTypeOf::String => message.get_value_string_utf8(),
          _ => "Unknown JavaScript Error".to_owned()
        }
      },
    };

    Self { message, handle: Some(handle) }
  }

  pub (crate) fn into_handle(self) -> napi::Handle {
    match self.handle {
      Some(handle) => handle,
      None => napi::env().create_error(&self.message),
    }
  }
}
