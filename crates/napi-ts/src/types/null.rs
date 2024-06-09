use crate::napi;
use crate::types::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct NapiNull<'a> {
  phantom: PhantomData<&'a ()>,
  handle: napi::Handle,
}

// impl Debug for NapiNull<'_> {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     f.debug_struct("NapiNull")
//       .field("@", &self.handle)
//       .finish()
//   }
// }

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiNull<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiNull<'a> {
  fn from_napi(_: napi::Env, handle: napi::Handle) -> Self {
    Self { phantom: PhantomData, handle }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== NULL ==================================================================

impl NapiFrom<()> for NapiNull<'_> {
  fn napi_from(_: (), env: napi::Env) -> Self {
    let handle = napi::get_null(env, );
    Self { phantom: PhantomData, handle }
  }
}
