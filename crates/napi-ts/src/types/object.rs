use crate::napi;
use crate::types::*;

pub struct NapiObject<'a> {
  handle: napi::Handle<'a>,
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
  fn from_napi_handle(handle: napi::Handle<'a>) -> Result<Self, NapiErr> {
    handle.expect_type_of(napi::TypeOf::napi_object)
      .map(|_| Self::from_napi_handle_unchecked(handle))
  }

  fn from_napi_handle_unchecked(handle: napi::Handle<'a>) -> Self {
    Self { handle }
  }

  fn napi_handle(&self) -> napi::Handle<'a> {
    self.handle
  }
}

// ===== OBJECT ================================================================

impl <'a> NapiFrom<'a, ()> for NapiObject<'a> {
  fn napi_from(_: (), env: napi::Env<'a>) -> Self {
    Self { handle: env.create_object() }
  }
}
