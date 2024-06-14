use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

napi_type!(NapiObject, Object, {
  handle: napi::Handle,
});

impl <'a> NapiProperties<'a> for NapiRef<'a, NapiObject> {}

impl NapiTypeInternal for NapiObject {
  #[inline]
  fn from_handle(handle: napi::Handle) -> Self {
    Self { handle }
  }

  #[inline]
  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== CONVERSION IN =========================================================

impl NapiObject {
  pub fn new(env: napi::Env) -> Self {
    Self { handle: env.create_object() }
  }
}
