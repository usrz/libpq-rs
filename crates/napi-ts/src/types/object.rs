use crate::types::*;

pub struct NapiObject {
  handle: napi::Handle,
}

// ===== NAPI TYPE BASICS ======================================================

napi_type!(NapiObject, Object);

impl <'a> NapiProperties<'a> for NapiRef<'a, NapiObject> {}

impl NapiTypeInternal for NapiObject {
  fn from_handle(handle: napi::Handle) -> Self {
    Self { handle }
  }

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
