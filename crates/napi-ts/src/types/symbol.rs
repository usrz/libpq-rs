use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiSymbol {
  handle: napi::Handle,
}

napi_type!(NapiSymbol, Symbol, {
  unsafe fn from_handle(handle: napi::Handle) -> Result<Self, NapiErr> {
    Ok(Self { handle })
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});

// ===== SYMBOL ================================================================

impl NapiSymbol {
  pub fn new(env: napi::Env, description: Option<&str>) -> Self {
    Self { handle: env.create_symbol(description)}
  }

  pub fn new_for(env: napi::Env, description: &str) -> Self {
    Self { handle: env.symbol_for(description)}
  }

  pub fn description(&self) -> Option<String> {
    let value = self.handle.get_named_property("description");
    match value.type_of() {
      NapiTypeOf::String => Some(value.get_value_string_utf8()),
      _ => None,
    }
  }
}
