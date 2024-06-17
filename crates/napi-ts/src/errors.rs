use crate::NapiRef;
use crate::NapiTypeOf;
use crate::napi;
use std::fmt;
use crate::NapiValue;
use crate::NapiTypeInternal;

// ========================================================================== //
// RESULT TYPE                                                                //
// ========================================================================== //

pub type NapiResult<'a, R> = Result<NapiRef<'a, R>, NapiErr>;

// impl<T, E> Result<T, E> {

pub trait NapiCatch<'a, T> {
  fn catch<F, O: FnOnce(NapiRef<'a, NapiValue<'a>>) -> Result<T, F>>(
    self, op: O
  ) -> Result<T, F>;
}

impl <'a, T> NapiCatch<'a, T> for Result<T, NapiErr> {
  fn catch<F, O: FnOnce(NapiRef<'a, NapiValue<'a>>) -> Result<T, F>>(self, op: O) -> Result<T, F> {
    match self {
      Err(e) => op(NapiValue::from_handle(e.into_handle()).as_napi_ref()),
      Ok(value) => Ok(value),
    }
  }
}

// ========================================================================== //
// "ERR" TYPE => holds an error message and an optional the napi value ptr    //
// ========================================================================== //

pub struct NapiErr {
  message: String,
  handle: napi::Handle,
}

impl fmt::Debug for NapiErr {
  fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
    fm.debug_tuple("NapiErr")
      .field(&self.handle)
      .field(&self.message)
      .finish()
  }
}

impl <T: AsRef<str>> From<T> for NapiErr {
  fn from(value: T) -> Self {
    let handle = napi::env().create_error(value.as_ref());
    Self { message: value.as_ref().to_string(), handle }
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

    Self { message, handle }
  }

  pub (crate) fn into_handle(self) -> napi::Handle {
    self.handle
  }
}
