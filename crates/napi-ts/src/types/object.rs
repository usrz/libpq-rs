use crate::napi;
use crate::types::*;

pub struct NapiObject {
  handle: napi::Handle,
}

// ===== NAPI TYPE BASICS ======================================================

napi_value!(NapiObject, Object);

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

impl <'a> NapiFrom<'a, ()> for NapiRef<'a, NapiObject> {
  fn napi_from(_: (), env: napi::Env) -> Self {
    NapiObject { handle: env.create_object() }.into()
  }
}
