use crate::napi;
use crate::types::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct NapiObject<'a> {
  phantom: PhantomData<&'a ()>,
  env: napi::Env,
  handle: napi::Handle,
}

// impl Debug for NapiObject<'_> {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     f.debug_struct("NapiObject")
//       .field("@", &self.handle)
//       .finish()
//   }
// }

// ===== NAPI::HANDLE CONVERSION ===============================================

impl NapiType for NapiObject<'_> {}

impl NapiFrom<napi::Handle> for NapiObject<'_> {
  fn napi_from(handle: napi::Handle, env: napi::Env) -> Self {
    Self { phantom: PhantomData, env, handle }
  }
}

impl NapiInto<napi::Handle> for NapiObject<'_> {
  fn napi_into(self, _env: napi::Env) -> napi::Handle {
    self.handle
  }
}

// ===== OBJECT ================================================================

impl NapiFrom<()> for NapiObject<'_> {
  fn napi_from(_: (), env: napi::Env) -> Self {
    let handle = napi::create_object();
    Self { phantom: PhantomData, env, handle }
  }
}
