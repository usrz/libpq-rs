use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiNull<'a> {
  phantom: PhantomData<&'a ()>,
  handle: napi::Handle,
}

napi_type!(NapiNull, Null, {
  unsafe fn from_handle(handle: napi::Handle) -> Result<Self, NapiErr> {
    Ok(Self { phantom: PhantomData, handle })
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});

// ===== NULL ==================================================================

impl <'a> NapiNull<'a> {
  pub fn new() -> Self {
    Self { phantom: PhantomData, handle: napi::env().get_null() }
  }
}
