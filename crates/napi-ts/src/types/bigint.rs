use crate::napi;
use crate::types::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct NapiBigint<'a> {
  phantom: PhantomData<&'a ()>,
  handle: napi::Handle,
  value: i128,
}

// impl Debug for NapiBigint<'_> {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     f.debug_struct("NapiBigint")
//       .field("@", &self.handle)
//       .finish()
//   }
// }

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiBigint<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiBigint<'a> {
  fn from_napi(env: napi::Env, handle: napi::Handle) -> Self {
    Self { phantom: PhantomData, handle, value: napi::get_value_bigint_words(env, handle) }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== I128 ==================================================================

impl NapiFrom<i128> for NapiBigint<'_> {
  fn napi_from(value: i128, env: napi::Env) -> Self {
    let handle = napi::create_bigint_words(env, value);
    Self { phantom: PhantomData, handle, value }
  }
}

impl Into<i128> for NapiBigint<'_> {
  fn into(self) -> i128 {
    self.value
  }
}

// ===== OTHER TYPES ===========================================================

impl NapiFrom<i8> for NapiBigint<'_> {
  fn napi_from(value: i8, env: napi::Env) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl NapiFrom<u8> for NapiBigint<'_> {
  fn napi_from(value: u8, env: napi::Env) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl NapiFrom<i16> for NapiBigint<'_> {
  fn napi_from(value: i16, env: napi::Env) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl NapiFrom<u16> for NapiBigint<'_> {
  fn napi_from(value: u16, env: napi::Env) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl NapiFrom<i32> for NapiBigint<'_> {
  fn napi_from(value: i32, env: napi::Env) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl NapiFrom<u32> for NapiBigint<'_> {
  fn napi_from(value: u32, env: napi::Env) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl NapiFrom<i64> for NapiBigint<'_> {
  fn napi_from(value: i64, env: napi::Env) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl NapiFrom<u64> for NapiBigint<'_> {
  fn napi_from(value: u64, env: napi::Env) -> Self {
    Self::napi_from(value as i128, env)
  }
}

// ===== EXTRA METHODS =========================================================

impl <'a> NapiBigint<'a> {
  pub fn value(&self) -> i128 {
    self.value
  }
}
