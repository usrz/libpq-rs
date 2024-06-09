use crate::napi;
use crate::types::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct NapiBoolean<'a> {
  phantom: PhantomData<&'a ()>,
  handle: napi::Handle,
  value: bool,
}

// impl Debug for NapiBool<'_> {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     f.debug_struct("NapiBool")
//       .field("@", &self.handle)
//       .finish()
//   }
// }

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiBoolean<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiBoolean<'a> {
  fn from_napi(env: napi::Env, handle: napi::Handle) -> Self {
    Self { phantom: PhantomData, handle, value: napi::get_value_bool(env, handle) }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== BOOL ==================================================================

impl NapiFrom<bool> for NapiBoolean<'_> {
  fn napi_from(value: bool, env: napi::Env) -> Self {
    let handle = napi::get_boolean(env, value);
    Self { phantom: PhantomData, handle, value }
  }
}

impl Into<bool> for NapiBoolean<'_> {
  fn into(self) -> bool {
    self.value
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiBoolean<'_> {
  pub fn value(&self) -> bool {
    self.value
  }
}
