
use crate::napi;
use crate::types::*;

pub (crate) enum Symbol {
  Symbol(Option<String>),
  SymbolFor(String),
}

pub struct NapiSymbol<'a> {
  handle: napi::Handle<'a>,
}

impl Debug for NapiSymbol<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiSymbol")
      .field("@", &self.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiSymbol<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiSymbol<'a> {
  fn from_napi_handle(handle: napi::Handle<'a>) -> Result<Self, NapiErr> {
    handle.expect_type_of(napi::TypeOf::napi_symbol)
      .map(|_| Self::from_napi_handle_unchecked(handle))
  }

  fn from_napi_handle_unchecked(handle: napi::Handle<'a>) -> Self {
    Self { handle }
  }

  fn napi_handle(&self) -> napi::Handle<'a> {
    self.handle
  }
}

// ===== STRING ================================================================

impl <'a> NapiFrom<'a, Symbol> for NapiSymbol<'a> {
  fn napi_from(value: Symbol, env: napi::Env<'a>) -> Self {
    Self {
      handle: match value {
        Symbol::SymbolFor(description) => env.symbol_for(&description),
        Symbol::Symbol(description) => {
          match description {
            Some(description) => env.create_symbol(Some(&description)),
            None => env.create_symbol(None),
          }
        }
      }
    }
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiSymbol<'_> {
  pub fn description(&self) -> Option<String> {
    let env = self.handle.env();
    let key = env.create_string_utf8("description");
    let value = env.get_property(&self.handle, &key);

    match self.handle.env().type_of(&value) {
      napi::TypeOf::napi_string => Some(env.get_value_string_utf8(&value)),
      _ => None,
    }
  }
}
