use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiObject<'a> {
  phantom: PhantomData<&'a ()>,
  handle: napi::Handle,
}

napi_type!(NapiObject, Object, {
  unsafe fn from_handle(handle: napi::Handle) -> Result<Self, NapiErr> {
    Ok(Self { phantom: PhantomData, handle })
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});

impl <'a> NapiProperties<'a> for NapiObject<'a> {}

// ===== CONVERSION IN =========================================================

impl <'a> NapiObject<'a> {
  pub fn new() -> Self {
    Self { phantom: PhantomData, handle: napi::env().create_object() }
  }
}
