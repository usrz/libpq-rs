use crate::napi;
use crate::types::*;

pub struct NapiBigint<'a> {
  handle: napi::Handle<'a>,
  value: i128,
}

impl Debug for NapiBigint<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiBigint")
      .field("@", &self.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiBigint<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiBigint<'a> {
  fn from_napi_handle(handle: napi::Handle<'a>) -> Result<Self, NapiErr> {
    handle.expect_type_of(napi::TypeOf::napi_bigint)
      .map(|_| Self::from_napi_handle_unchecked(handle))
  }

  fn from_napi_handle_unchecked(handle: napi::Handle<'a>) -> Self {
    let value = handle.get_value_bigint_words();
    Self { handle, value }
  }

  fn napi_handle(&self) -> napi::Handle<'a> {
    self.handle
  }
}

// ===== I128 ==================================================================

impl <'a> NapiFrom<'a, i128> for NapiBigint<'a> {
  fn napi_from(value: i128, env: napi::Env<'a>) -> Self {
    Self { handle: env.create_bigint_words(value), value }
  }
}

impl Into<i128> for NapiBigint<'_> {
  fn into(self) -> i128 {
    self.value
  }
}

// ===== OTHER TYPES ===========================================================

impl <'a> NapiFrom<'a, i8> for NapiBigint<'a> {
  fn napi_from(value: i8, env: napi::Env<'a>) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl <'a> NapiFrom<'a, u8> for NapiBigint<'a> {
  fn napi_from(value: u8, env: napi::Env<'a>) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl <'a> NapiFrom<'a, i16> for NapiBigint<'a> {
  fn napi_from(value: i16, env: napi::Env<'a>) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl <'a> NapiFrom<'a, u16> for NapiBigint<'a> {
  fn napi_from(value: u16, env: napi::Env<'a>) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl <'a> NapiFrom<'a, i32> for NapiBigint<'a> {
  fn napi_from(value: i32, env: napi::Env<'a>) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl <'a> NapiFrom<'a, u32> for NapiBigint<'a> {
  fn napi_from(value: u32, env: napi::Env<'a>) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl <'a> NapiFrom<'a, i64> for NapiBigint<'a> {
  fn napi_from(value: i64, env: napi::Env<'a>) -> Self {
    Self::napi_from(value as i128, env)
  }
}

impl <'a> NapiFrom<'a, u64> for NapiBigint<'a> {
  fn napi_from(value: u64, env: napi::Env<'a>) -> Self {
    Self::napi_from(value as i128, env)
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiBigint<'_> {
  pub fn value(&self) -> i128 {
    self.value
  }
}
