use crate::napi;
use crate::types::*;

pub struct NapiUndefined<'a> {
  handle: NapiHandle<'a>,
}

impl Debug for NapiUndefined<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiUndefined")
      .field("@", &self.handle.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiUndefined<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiUndefined<'a> {
  fn from_napi_handle(handle: NapiHandle<'a>) -> Result<Self, NapiErr> {
    napi::expect_type_of(handle.env, handle.handle, napi::TypeOf::napi_undefined)
      .map(|_| Self::from_napi_handle_unchecked(handle))
  }

  fn from_napi_handle_unchecked(handle: NapiHandle<'a>) -> Self {
    Self { handle }
  }

  fn get_napi_handle(&self) -> &NapiHandle<'a> {
    &self.handle
  }

  fn into_napi_handle(self) -> NapiHandle<'a> {
    self.handle
  }
}

// ===== UNDEFINED =============================================================

impl NapiFrom<()> for NapiUndefined<'_> {
  fn napi_from(_: (), env: napi::Env) -> Self {
    let handle = napi::get_undefined(env);
    Self { handle: NapiHandle::from_napi(env, handle) }
  }
}
