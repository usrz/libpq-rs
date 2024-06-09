use crate::napi;
use crate::types::*;
use std::marker::PhantomData;

pub struct NapiObject<'a> {
  phantom: PhantomData<&'a ()>,
  env: napi::Env,
  handle: napi::Handle,
}

impl Debug for NapiObject<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiObject")
      .field("@", &self.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiObject<'a> {}
impl <'a> NapiProperties<'a> for NapiObject<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiObject<'a> {
  fn from_napi(env: napi::Env, handle: napi::Handle) -> Self {
    Self { phantom: PhantomData, env, handle }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

impl <'a> NapiPropertiesInternal<'a> for NapiObject<'a> {
  fn napi_env(&self) -> napi::Env {
    self.env
  }
}

// ===== OBJECT ================================================================

impl NapiFrom<()> for NapiObject<'_> {
  fn napi_from(_: (), env: napi::Env) -> Self {
    let handle = napi::create_object(env);
    Self { phantom: PhantomData, env, handle }
  }
}
