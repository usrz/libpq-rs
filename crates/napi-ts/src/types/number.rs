use crate::napi;
use crate::types::*;

pub struct NapiNumber {
  handle: napi::Handle,
  value: f64,
}

// ===== NAPI TYPE BASICS ======================================================

napi_value!(NapiNumber, Number);

impl NapiTypeInternal for NapiNumber {
  fn from_handle(handle: napi::Handle) -> Self {
    Self { handle, value: handle.get_value_double() }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== CONVERSION OUT ========================================================

impl NapiNumber {
  pub fn value(&self) -> f64 {
    self.value
  }
}

// ===== CONVERSION IN =========================================================

impl <'a> NapiFrom<'a, f64> for NapiRef<'a, NapiNumber> {
  fn napi_from(value: f64, env: napi::Env) -> Self {
    NapiNumber { handle: env.create_double(value), value }.into()
  }
}

// ===== OTHER TYPES ===========================================================

impl <'a> NapiFrom<'a, i8> for NapiRef<'a, NapiNumber> {
  fn napi_from(value: i8, env: napi::Env) -> Self {
    Self::napi_from(value as f64, env)
  }
}

impl <'a> NapiFrom<'a, u8> for NapiRef<'a, NapiNumber> {
  fn napi_from(value: u8, env: napi::Env) -> Self {
    Self::napi_from(value as f64, env)
  }
}

impl <'a> NapiFrom<'a, i16> for NapiRef<'a, NapiNumber> {
  fn napi_from(value: i16, env: napi::Env) -> Self {
    Self::napi_from(value as f64, env)
  }
}

impl <'a> NapiFrom<'a, u16> for NapiRef<'a, NapiNumber> {
  fn napi_from(value: u16, env: napi::Env) -> Self {
    Self::napi_from(value as f64, env)
  }
}

impl <'a> NapiFrom<'a, i32> for NapiRef<'a, NapiNumber> {
  fn napi_from(value: i32, env: napi::Env) -> Self {
    Self::napi_from(value as f64, env)
  }
}

impl <'a> NapiFrom<'a, u32> for NapiRef<'a, NapiNumber> {
  fn napi_from(value: u32, env: napi::Env) -> Self {
    Self::napi_from(value as f64, env)
  }
}

impl <'a> NapiFrom<'a, f32> for NapiRef<'a, NapiNumber> {
  fn napi_from(value: f32, env: napi::Env) -> Self {
    Self::napi_from(value as f64, env)
  }
}
