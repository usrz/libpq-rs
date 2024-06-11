use crate::napi;
use crate::types::*;

pub struct NapiObject<'a> {
  handle: napi::Handle<'a>,
}

// ===== NAPI TYPE BASICS ======================================================

napi_type!(NapiObject, Object);

impl <'a> NapiProperties<'a> for NapiObject<'a> {}

impl <'a> TryFrom<NapiValue<'a>> for NapiObject<'a> {
  type Error = NapiErr;

  fn try_from(value: NapiValue<'a>) -> Result<Self, Self::Error> {
    match value {
      NapiValue::Object(handle) => Ok(Self { handle }),
      _ => Err(format!("Can't downcast {} into NapiObject", value).into()),
    }
  }
}

// ===== OBJECT ================================================================

impl <'a> NapiFrom<'a, ()> for NapiObject<'a> {
  fn napi_from(_: (), env: napi::Env<'a>) -> Self {
    Self { handle: env.create_object() }
  }
}
