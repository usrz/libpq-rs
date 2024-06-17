use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiUndefined<'a> {
  phantom: PhantomData<&'a ()>,
  handle: napi::Handle,
}

napi_type!(NapiUndefined, Undefined, {
  unsafe fn from_handle(handle: napi::Handle) -> Result<Self, NapiErr> {
    Ok(Self { phantom: PhantomData, handle })
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});

// ===== UNDEFINED =============================================================

impl <'a> NapiUndefined<'a> {
  pub fn new() -> Self {
    Self { phantom: PhantomData, handle: napi::env().get_undefined() }
  }
}
