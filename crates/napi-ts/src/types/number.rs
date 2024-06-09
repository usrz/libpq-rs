use crate::napi;
use crate::types::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct NapiNumber<'a> {
  phantom: PhantomData<&'a ()>,
  handle: napi::Handle,
  value: f64,
}

// impl Debug for NapiNumber<'_> {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     f.debug_struct("NapiNumber")
//       .field("@", &self.handle)
//       .finish()
//   }
// }

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiNumber<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiNumber<'a> {
  fn from_napi(env: napi::Env, handle: napi::Handle) -> Self {
    Self { phantom: PhantomData, handle, value: napi::get_value_double(env, handle) }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== F64 ==================================================================

impl NapiFrom<f64> for NapiNumber<'_> {
  fn napi_from(value: f64, env: napi::Env) -> Self {
    let handle = napi::create_double(env, value);
    Self { phantom: PhantomData, handle, value }
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
