use crate::napi;
use crate::types::*;

pub struct NapiBigint {
  handle: napi::Handle,
  value: i128,
}

// ===== NAPI TYPE BASICS ======================================================

napi_value!(NapiBigint, Bigint);

impl NapiTypeInternal for NapiBigint {
  #[inline]
  fn from_handle(handle: napi::Handle) -> Self {
    Self { handle, value: handle.get_value_bigint_words() }
  }

  #[inline]
  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== CONVERSION OUT ========================================================

impl NapiBigint {
  pub fn value(&self) -> i128 {
    self.value
  }
}

// ===== CONVERSION IN =========================================================

impl <'a> NapiFrom<'a, i128> for NapiRef<'a, NapiBigint> {
  fn napi_from(value: i128, env: napi::Env) -> Self {
    NapiBigint { handle: env.create_bigint_words(value), value }.into()
  }
}

// ===== OTHER TYPES ===========================================================

impl <'a> NapiFrom<'a, i8> for NapiRef<'a, NapiBigint> {
  fn napi_from(value: i8, env: napi::Env) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl <'a> NapiFrom<'a, u8> for NapiRef<'a, NapiBigint> {
  fn napi_from(value: u8, env: napi::Env) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl <'a> NapiFrom<'a, i16> for NapiRef<'a, NapiBigint> {
  fn napi_from(value: i16, env: napi::Env) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl <'a> NapiFrom<'a, u16> for NapiRef<'a, NapiBigint> {
  fn napi_from(value: u16, env: napi::Env) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl <'a> NapiFrom<'a, i32> for NapiRef<'a, NapiBigint> {
  fn napi_from(value: i32, env: napi::Env) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl <'a> NapiFrom<'a, u32> for NapiRef<'a, NapiBigint> {
  fn napi_from(value: u32, env: napi::Env) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl <'a> NapiFrom<'a, i64> for NapiRef<'a, NapiBigint> {
  fn napi_from(value: i64, env: napi::Env) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl <'a> NapiFrom<'a, u64> for NapiRef<'a, NapiBigint> {
  fn napi_from(value: u64, env: napi::Env) -> Self {
    Self::napi_from(value as i128, env)
  }
}
