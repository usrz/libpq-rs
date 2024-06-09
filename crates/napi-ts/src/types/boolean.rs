use crate::napi;
use crate::types::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct NapiBoolean<'a> {
  phantom: PhantomData<&'a ()>,
  env: napi::Env,
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

impl NapiType for NapiBoolean<'_> {}

impl NapiTypeInternal for NapiBoolean<'_> {
  fn handle(&self) -> napi::Handle {
    self.handle
  }
}

impl NapiFrom<napi::Handle> for NapiBoolean<'_> {
  fn napi_from(handle: napi::Handle, env: napi::Env) -> Self {
    Self { phantom: PhantomData, env, handle, value: napi::get_value_bool(env, handle) }
  }
}

impl NapiInto<napi::Handle> for NapiBoolean<'_> {
  fn napi_into(self, _env: napi::Env) -> napi::Handle {
    self.handle
  }
}

// ===== BOOL ==================================================================

impl NapiFrom<bool> for NapiBoolean<'_> {
  fn napi_from(value: bool, env: napi::Env) -> Self {
    let handle = napi::get_boolean(env, value);
    Self { phantom: PhantomData, env, handle, value }
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
