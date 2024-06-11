use crate::napi;
use crate::types::*;

pub struct NapiBoolean<'a> {
  handle: napi::Handle<'a>,
  value: bool,
}

impl Debug for NapiBoolean<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiBoolean")
      .field("@", &self.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

napi_type!(NapiBoolean, Boolean);

impl <'a> TryFrom<NapiValue<'a>> for NapiBoolean<'a> {
  type Error = NapiErr;

  fn try_from(value: NapiValue<'a>) -> Result<Self, Self::Error> {
    match value {
      NapiValue::Boolean(handle) => Ok(Self { handle, value: handle.get_value_bool() }),
      _ => Err(format!("Can't downcast {} into NapiBoolean", value).into()),
    }
  }
}

impl <'a> NapiBoolean<'a> {
  pub fn value(&self) -> bool {
    self.value
  }
}

// ===== BOOL ==================================================================

impl <'a> NapiFrom<'a, bool> for NapiBoolean<'a> {
  fn napi_from(value: bool, env: napi::Env<'a>) -> Self {
    Self { handle: env.get_boolean(value), value }
  }
}

impl Into<bool> for NapiBoolean<'_> {
  fn into(self) -> bool {
    self.value
  }
}
