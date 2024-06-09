use crate::napi;
use crate::types::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct NapiString<'a> {
  phantom: PhantomData<&'a ()>,
  handle: napi::Handle,
  value: String,
}

// impl Debug for NapiString<'_> {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     f.debug_struct("NapiString")
//       .field("@", &self.handle)
//       .finish()
//   }
// }

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiString<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiString<'a> {
  fn from_napi(env: napi::Env, handle: napi::Handle) -> Self {
    Self { phantom: PhantomData, handle, value: napi::get_value_string_utf8(env, handle) }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== STRING ================================================================

impl NapiFrom<&str> for NapiString<'_> {
  fn napi_from(value: &str, env: napi::Env) -> Self {
    let handle = napi::create_string_utf8(env, value);
    Self { phantom: PhantomData, handle, value: value.to_string() }
  }
}

impl Into<String> for NapiString<'_> {
  fn into(self) -> String {
    self.value
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiString<'_> {
  pub fn value(&self) -> String {
    self.value.clone()
  }
}
