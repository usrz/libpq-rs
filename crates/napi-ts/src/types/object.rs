use crate::napi;
use crate::types::*;

pub struct NapiObject<'a> {
  handle: NapiHandle<'a>,
}

impl Debug for NapiObject<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiObject")
      .field("@", &self.handle.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiObject<'a> {}
impl <'a> NapiProperties<'a> for NapiObject<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiObject<'a> {
  fn from_napi_handle(handle: NapiHandle<'a>) -> Result<Self, NapiErr> {
    napi::expect_type_of(handle.env, handle.handle, napi::TypeOf::napi_object)
      .map(|_| Self::from_napi_handle_unchecked(handle))
  }

  fn from_napi_handle_unchecked(handle: NapiHandle<'a>) -> Self {
    Self { handle }
  }

  fn get_napi_handle(&self) -> &NapiHandle<'a> {
    &self.handle
  }
}

// ===== OBJECT ================================================================

impl NapiFrom<()> for NapiObject<'_> {
  fn napi_from(_: (), env: napi::Env) -> Self {
    let handle = napi::create_object(env);
    Self { handle: NapiHandle::from_napi(env, handle) }
  }
}
