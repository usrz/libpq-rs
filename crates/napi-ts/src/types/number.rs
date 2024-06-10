use crate::napi;
use crate::types::*;

pub struct NapiNumber<'a> {
  handle: NapiHandle<'a>,
  value: f64,
}

impl Debug for NapiNumber<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiNumber")
      .field("@", &self.handle.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiNumber<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiNumber<'a> {
  fn from_napi_handle(handle: NapiHandle<'a>) -> Result<Self, NapiErr> {
    napi::expect_type_of(handle.env, handle.handle, napi::TypeOf::napi_number)
      .map(|_| Self::from_napi_handle_unchecked(handle))
  }

  fn from_napi_handle_unchecked(handle: NapiHandle<'a>) -> Self {
    let value = napi::get_value_double(handle.env, handle.handle);
    Self { handle, value }
  }

  fn get_napi_handle(&self) -> &NapiHandle<'a> {
    &self.handle
  }
}

// ===== F64 ==================================================================

impl NapiFrom<f64> for NapiNumber<'_> {
  fn napi_from(value: f64, env: napi::Env) -> Self {
    let handle = napi::create_double(env, value);
    Self { handle: NapiHandle::from_napi(env, handle), value }
  }
}

impl Into<f64> for NapiNumber<'_> {
  fn into(self) -> f64 {
    self.value
  }
}

// ===== OTHER TYPES ===========================================================

impl NapiFrom<i8> for NapiNumber<'_> {
  fn napi_from(value: i8, env: napi::Env) -> Self {
    Self::napi_from(value as f64, env)
  }
}

impl NapiFrom<u8> for NapiNumber<'_> {
  fn napi_from(value: u8, env: napi::Env) -> Self {
    Self::napi_from(value as f64, env)
  }
}

impl NapiFrom<i16> for NapiNumber<'_> {
  fn napi_from(value: i16, env: napi::Env) -> Self {
    Self::napi_from(value as f64, env)
  }
}

impl NapiFrom<u16> for NapiNumber<'_> {
  fn napi_from(value: u16, env: napi::Env) -> Self {
    Self::napi_from(value as f64, env)
  }
}

impl NapiFrom<i32> for NapiNumber<'_> {
  fn napi_from(value: i32, env: napi::Env) -> Self {
    Self::napi_from(value as f64, env)
  }
}

impl NapiFrom<u32> for NapiNumber<'_> {
  fn napi_from(value: u32, env: napi::Env) -> Self {
    Self::napi_from(value as f64, env)
  }
}

impl NapiFrom<f32> for NapiNumber<'_> {
  fn napi_from(value: f32, env: napi::Env) -> Self {
    Self::napi_from(value as f64, env)
  }
}

// ===== EXTRA METHODS =========================================================

impl <'a> NapiNumber<'a> {
  pub fn value(&self) -> f64 {
    self.value
  }
}
