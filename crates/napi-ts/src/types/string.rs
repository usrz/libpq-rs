use crate::napi;
use crate::types::*;

pub struct NapiString<'a> {
  handle: napi::Handle<'a>,
  value: String,
}

// ===== NAPI TYPE BASICS ======================================================

napi_type!(NapiString, String);

impl <'a> TryFrom<NapiValue<'a>> for NapiString<'a> {
  type Error = NapiErr;

  fn try_from(value: NapiValue<'a>) -> Result<Self, Self::Error> {
    match value {
      NapiValue::String(handle) => Ok(Self { handle, value: handle.get_value_string_utf8() }),
      _ => Err(format!("Can't downcast {} into NapiString", value).into()),
    }
  }
}

// ===== STRING ================================================================

impl <'a> NapiFrom<'a, &str> for NapiString<'a> {
  fn napi_from(value: &str, env: napi::Env<'a>) -> Self {
    let handle = env.create_string_utf8(value);
    Self { handle, value: value.to_string() }
  }
}

impl Into<String> for NapiString<'_> {
  fn into(self) -> String {
    self.value
  }
}
