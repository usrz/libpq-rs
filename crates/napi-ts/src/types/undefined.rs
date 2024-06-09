use crate::napi;
use crate::types::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct NapiUndefined<'a> {
  phantom: PhantomData<&'a ()>,
  env: napi::Env,
  handle: napi::Handle,
}

// impl Debug for NapiUndefined<'_> {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     f.debug_struct("NapiUndefined")
//       .field("@", &self.handle)
//       .finish()
//   }
// }

// ===== NAPI::HANDLE CONVERSION ===============================================

impl NapiType for NapiUndefined<'_> {}

impl NapiTypeInternal for NapiUndefined<'_> {
  fn handle(&self) -> napi::Handle {
    self.handle
  }
}

impl NapiFrom<napi::Handle> for NapiUndefined<'_> {
  fn napi_from(handle: napi::Handle, env: napi::Env) -> Self {
    Self { phantom: PhantomData, env, handle }
  }
}

impl NapiInto<napi::Handle> for NapiUndefined<'_> {
  fn napi_into(self, _env: napi::Env) -> napi::Handle {
    self.handle
  }
}

// ===== NULL ==================================================================

impl NapiFrom<()> for NapiUndefined<'_> {
  fn napi_from(_: (), env: napi::Env) -> Self {
    let handle = napi::get_null(env);
    Self { phantom: PhantomData, env, handle }
  }
}
