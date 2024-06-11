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

impl <'a> NapiType<'a> for NapiNumber<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiNumber<'a> {
  fn from_napi_handle(handle: napi::Handle<'a>) -> Result<Self, NapiErr> {
    handle.expect_type_of(napi::TypeOf::napi_number)
      .map(|_| Self::from_napi_handle_unchecked(handle))
  }

  fn from_napi_handle_unchecked(handle: napi::Handle<'a>) -> Self {
    let value = handle.get_value_double();
    Self { handle, value }
  }

  fn napi_handle(&self) -> napi::Handle<'a> {
    self.handle
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

// ===== EXTRA METHODS =========================================================

impl NapiNumber<'_> {
  pub fn value(&self) -> f64 {
    self.value
  }
}
