use crate::napi;
use crate::types::*;

pub struct NapiUndefined<'a> {
  handle: napi::Handle<'a>,
}

// ===== NAPI TYPE BASICS ======================================================

napi_type!(NapiUndefined, Undefined);

impl <'a> TryFrom<NapiValue<'a>> for NapiUndefined<'a> {
  type Error = NapiErr;

  fn try_from(value: NapiValue<'a>) -> Result<Self, Self::Error> {
    match value {
      NapiValue::Undefined(handle) => Ok(Self { handle }),
      _ => Err(format!("Can't downcast {} into NapiUndefined", value).into()),
    }
  }
}


// ===== UNDEFINED =============================================================

impl <'a> NapiFrom<'a, ()> for NapiUndefined<'a> {
  fn napi_from(_: (), env: napi::Env<'a>) -> Self {
    Self { handle: env.get_undefined() }
  }
}
