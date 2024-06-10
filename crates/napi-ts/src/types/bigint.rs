use crate::napi;
use crate::types::*;

pub struct NapiBigint<'a> {
  handle: NapiHandle<'a>,
  value: i128,
}

impl Debug for NapiBigint<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiBigint")
      .field("@", &self.handle.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiBigint<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiBigint<'a> {
  fn from_napi_handle(handle: NapiHandle<'a>) -> Result<Self, NapiErr> {
    napi::expect_type_of(handle.env, handle.handle, napi::TypeOf::napi_bigint)
      .map(|_| Self::from_napi_handle_unchecked(handle))
  }

  fn from_napi_handle_unchecked(handle: NapiHandle<'a>) -> Self {
    let value = napi::get_value_bigint_words(handle.env, handle.handle);
    Self { handle, value }
  }

  fn get_napi_handle(&self) -> &NapiHandle<'a> {
    &self.handle
  }
}

// ===== I128 ==================================================================

impl NapiFrom<i128> for NapiBigint<'_> {
  fn napi_from(value: i128, env: napi::Env) -> Self {
    let handle = napi::create_bigint_words(env, value);
    Self { handle: NapiHandle::from_napi(env, handle), value }
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
