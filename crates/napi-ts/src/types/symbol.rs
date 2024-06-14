
use crate::napi;
use crate::types::*;

pub struct NapiSymbol {
  handle: napi::Handle,
}

// ===== NAPI TYPE BASICS ======================================================

napi_type!(NapiSymbol, Symbol);

impl NapiTypeInternal for NapiSymbol {
  fn from_handle(handle: napi::Handle) -> Self {
    Self { handle }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

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
      napi::TypeOf::String => Some(value.get_value_string_utf8()),
      _ => None,
    }
  }
}
