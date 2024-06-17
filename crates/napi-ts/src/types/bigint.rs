use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiBigint<'a> {
  phantom: PhantomData<&'a ()>,
  handle: napi::Handle,
  value: i128,
}

napi_type!(NapiBigint, Bigint, {
  unsafe fn from_handle(handle: napi::Handle) -> Result<Self, NapiErr> {
    Ok(Self { phantom: PhantomData, handle, value: handle.get_value_bigint_words() })
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});


// ===== BIGINT ================================================================

impl <'a> NapiBigint<'a> {
  pub fn new(value: i128) -> Self {
    Self { phantom: PhantomData, handle: napi::env().create_bigint_words(value), value }
  }

  pub fn value(&self) -> i128 {
    self.value
  }
}
