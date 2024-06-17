use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiSymbol<'a> {
  phantom: PhantomData<&'a ()>,
  handle: napi::Handle,
}

napi_type!(NapiSymbol, Symbol, {
  unsafe fn from_handle(handle: napi::Handle) -> Result<Self, NapiErr> {
    Ok(Self { phantom: PhantomData, handle })
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});

// ===== SYMBOL ================================================================

impl <'a> NapiSymbol<'a> {
  pub fn new(description: Option<&str>) -> Self {
    Self { phantom: PhantomData, handle: napi::env().create_symbol(description)}
  }

  pub fn new_for(description: &str) -> Self {
    Self { phantom: PhantomData, handle: napi::env().symbol_for(description)}
  }

  pub fn description(&self) -> Option<String> {
    let value = self.handle.get_named_property("description");
    match value.type_of() {
      NapiTypeOf::String => Some(value.get_value_string_utf8()),
      _ => None,
    }
  }
}
