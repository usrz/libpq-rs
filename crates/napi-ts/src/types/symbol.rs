
use crate::napi;
use crate::types::*;

pub (crate) enum NapiSymbolInternal {
  Symbol(Option<String>),
  SymbolFor(String),
}

pub struct NapiSymbol {
  handle: napi::Handle,
}

// ===== NAPI TYPE BASICS ======================================================

napi_value!(NapiSymbol, Symbol);

impl NapiTypeInternal for NapiSymbol {
  fn from_handle(handle: napi::Handle) -> Self {
    Self { handle }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== SYMBOL ================================================================

impl <'a> NapiFrom<'a, NapiSymbolInternal> for NapiRef<'a, NapiSymbol> {
  fn napi_from(value: NapiSymbolInternal, env: napi::Env) -> Self {
    let handle = match value {
      NapiSymbolInternal::SymbolFor(description) => env.symbol_for(&description),
      NapiSymbolInternal::Symbol(description) => {
        match description {
          Some(description) => env.create_symbol(Some(&description)),
          None => env.create_symbol(None),
        }
      }
    };

    NapiSymbol { handle }.into()
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiSymbol {
  pub fn description(&self) -> Option<String> {
    let value = self.handle.get_named_property("description");
    match value.type_of() {
      napi::TypeOf::String => Some(value.get_value_string_utf8()),
      _ => None,
    }
  }
}
