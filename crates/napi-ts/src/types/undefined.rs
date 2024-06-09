use crate::napi;
use crate::types::*;
use std::marker::PhantomData;

pub struct NapiUndefined<'a> {
  phantom: PhantomData<&'a ()>,
  handle: napi::Handle,
}

impl Debug for NapiUndefined<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiUndefined")
      .field("@", &self.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiUndefined<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiUndefined<'a> {
  fn from_napi(_: napi::Env, handle: napi::Handle) -> Self {
    Self { phantom: PhantomData, handle }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== NULL ==================================================================

impl NapiFrom<()> for NapiUndefined<'_> {
  fn napi_from(_: (), env: napi::Env) -> Self {
    let handle = napi::get_null(env);
    Self { phantom: PhantomData, handle }
  }
}
