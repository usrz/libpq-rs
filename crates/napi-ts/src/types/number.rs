use crate::napi;
use crate::types::*;

pub struct NapiNumber<'a> {
  handle: napi::Handle<'a>,
  value: f64,
}

impl Debug for NapiNumber<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiNumber")
      .field("@", &self.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiNumber<'a> {
  fn napi_handle(&self) -> napi::Handle<'a> {
    self.handle
  }
}

impl <'a> TryFrom<NapiValue<'a>> for NapiNumber<'a> {
  type Error = NapiErr;

  fn try_from(value: NapiValue<'a>) -> Result<Self, Self::Error> {
    match value {
      NapiValue::Number(handle) => Ok(Self { handle, value: handle.get_value_double() }),
      _ => Err(format!("Can't downcast {} into NapiBoolean", value).into()),
    }
  }
}

impl <'a> NapiNumber<'a> {
  pub fn value(&self) -> f64 {
    self.value
  }
}

// ===== F64 ==================================================================

impl <'a> NapiFrom<'a, f64> for NapiNumber<'a> {
  fn napi_from(value: f64, env: napi::Env<'a>) -> Self {
    Self { handle: env.create_double(value), value }
  }
}

impl Into<f64> for NapiNumber<'_> {
  fn into(self) -> f64 {
    self.value
  }
}

// ===== OTHER TYPES ===========================================================

impl <'a> NapiFrom<'a, i8> for NapiNumber<'a> {
  fn napi_from(value: i8, env: napi::Env<'a>) -> Self {
    Self::napi_from(value as f64, env)
  }
}

impl <'a> NapiFrom<'a, u8> for NapiNumber<'a> {
  fn napi_from(value: u8, env: napi::Env<'a>) -> Self {
    Self::napi_from(value as f64, env)
  }
}

impl <'a> NapiFrom<'a, i16> for NapiNumber<'a> {
  fn napi_from(value: i16, env: napi::Env<'a>) -> Self {
    Self::napi_from(value as f64, env)
  }
}

impl <'a> NapiFrom<'a, u16> for NapiNumber<'a> {
  fn napi_from(value: u16, env: napi::Env<'a>) -> Self {
    Self::napi_from(value as f64, env)
  }
}

impl <'a> NapiFrom<'a, i32> for NapiNumber<'a> {
  fn napi_from(value: i32, env: napi::Env<'a>) -> Self {
    Self::napi_from(value as f64, env)
  }
}

impl <'a> NapiFrom<'a, u32> for NapiNumber<'a> {
  fn napi_from(value: u32, env: napi::Env<'a>) -> Self {
    Self::napi_from(value as f64, env)
  }
}

impl <'a> NapiFrom<'a, f32> for NapiNumber<'a> {
  fn napi_from(value: f32, env: napi::Env<'a>) -> Self {
    Self::napi_from(value as f64, env)
  }
}
