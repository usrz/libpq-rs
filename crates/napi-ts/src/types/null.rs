use crate::napi;
use crate::types::*;

pub struct NapiNull<'a> {
  handle: napi::Handle<'a>,
}

impl Debug for NapiNull<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiNull")
      .field("@", &self.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiNull<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiNull<'a> {
  fn from_napi_handle(handle: napi::Handle<'a>) -> Result<Self, NapiErr> {
    handle.expect_type_of(napi::TypeOf::napi_null)
      .map(|_| Self::from_napi_handle_unchecked(handle))
  }

  fn from_napi_handle_unchecked(handle: napi::Handle<'a>) -> Self {
    Self { handle }
  }

  fn napi_handle(&self) -> napi::Handle<'a> {
    self.handle
  }
}

// ===== NULL ==================================================================

impl <'a> NapiFrom<'a, ()> for NapiNull<'a> {
  fn napi_from(_: (), env: napi::Env<'a>) -> Self {
    Self { handle: env.get_null() }
  }
}
