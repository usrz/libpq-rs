use crate::napi;
use crate::types::*;

pub struct NapiNull<'a> {
  handle: napi::Handle<'a>,
}

impl Debug for NapiNull<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiNull")
      .field("@", &self.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiNull<'a> {
  fn napi_handle(&self) -> napi::Handle<'a> {
    self.handle
  }
}

impl <'a> TryFrom<NapiValue<'a>> for NapiNull<'a> {
  type Error = NapiErr;

  fn try_from(value: NapiValue<'a>) -> Result<Self, Self::Error> {
    match value {
      NapiValue::Null(handle) => Ok(Self { handle }),
      _ => Err(format!("Can't downcast {} into NapiNull", value).into()),
    }
  }
}

// ===== NULL ==================================================================

impl <'a> NapiFrom<'a, ()> for NapiNull<'a> {
  fn napi_from(_: (), env: napi::Env<'a>) -> Self {
    Self { handle: env.get_null() }
  }
}
