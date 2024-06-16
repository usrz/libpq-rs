use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiObject {
  handle: napi::Handle,
}

napi_type!(NapiObject, Object, {
  unsafe fn from_handle(handle: napi::Handle) -> Result<Self, NapiErr> {
    Ok(Self { handle })
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});

impl <'a> NapiProperties<'a> for NapiRef<'a, NapiObject> {}

// ===== CONVERSION IN =========================================================

impl NapiObject {
  pub fn new(env: napi::Env) -> Self {
    Self { handle: env.create_object() }
  }
}
