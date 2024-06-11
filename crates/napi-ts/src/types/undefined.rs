use crate::napi;
use crate::types::*;

pub struct NapiUndefined<'a> {
  handle: napi::Handle<'a>,
}

impl Debug for NapiUndefined<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiUndefined")
      .field("@", &self.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiUndefined<'a> {
  fn napi_handle(&self) -> napi::Handle<'a> {
    self.handle
  }
}

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
