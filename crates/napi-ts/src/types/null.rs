use crate::napi;
use crate::types::*;

pub struct NapiNull<'a> {
  handle: NapiHandle<'a>,
}

impl Debug for NapiNull<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiNull")
      .field("@", &self.handle.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiNull<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiNull<'a> {
  fn from_napi_handle(handle: NapiHandle<'a>) -> Result<Self, NapiErr> {
    napi::expect_type_of(handle.env, handle.handle, napi::TypeOf::napi_null)
      .map(|_| Self::from_napi_handle_unchecked(handle))
  }

  fn from_napi_handle_unchecked(handle: NapiHandle<'a>) -> Self {
    Self { handle }
  }

  fn get_napi_handle(&self) -> &NapiHandle<'a> {
    &self.handle
  }
}

// ===== NULL ==================================================================

impl NapiFrom<()> for NapiNull<'_> {
  fn napi_from(_: (), env: napi::Env) -> Self {
    let handle = napi::get_null(env);
    Self { handle: NapiHandle::from_napi(env, handle) }
  }
}
