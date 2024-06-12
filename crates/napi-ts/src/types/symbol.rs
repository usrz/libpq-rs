
use crate::napi;
use crate::types::*;

pub (crate) enum NapiSymbolInternal {
  Symbol(Option<String>),
  SymbolFor(String),
}

pub struct NapiSymbol<'a> {
  handle: napi::Handle<'a>,
}

// ===== NAPI TYPE BASICS ======================================================

napi_type!(NapiSymbol, Symbol);

impl <'a> TryFrom<NapiValue<'a>> for NapiSymbol<'a> {
  type Error = NapiErr;

  fn try_from(value: NapiValue<'a>) -> Result<Self, Self::Error> {
    match value {
      NapiValue::Symbol(handle) => Ok(Self { handle }),
      _ => Err(format!("Can't downcast {} into NapiSymbol", value).into()),
    }
  }
}

// ===== SYMBOL ================================================================

impl <'a> NapiFrom<'a, NapiSymbolInternal> for NapiSymbol<'a> {
  fn napi_from(value: NapiSymbolInternal, env: napi::Env<'a>) -> Self {
    Self {
      handle: match value {
        NapiSymbolInternal::SymbolFor(description) => env.symbol_for(&description),
        NapiSymbolInternal::Symbol(description) => {
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
      napi::TypeOf::String => Some(env.get_value_string_utf8(&value)),
      _ => None,
    }
  }
}
