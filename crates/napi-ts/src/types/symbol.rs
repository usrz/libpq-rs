use crate::napi;
use crate::types::*;
use crate::errors::NapiResult;

#[derive(Clone,Debug)]
pub struct NapiSymbol {
  value: napi::Value,
}

impl NapiValue for NapiSymbol {}

impl NapiValueInternal for NapiSymbol {
  fn as_napi_value(&self) -> napi::Value {
    self.value
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { value }
  }
}

impl Into<NapiResult<NapiValues>> for NapiSymbol {
  fn into(self) -> NapiResult<NapiValues> {
    Ok(self.into())
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiSymbol {
  pub fn new(description: &str) -> Self {
    let value = napi::create_string_utf8(description);
    Self { value: napi::create_symbol(value) }
  }

  pub fn symbol_for(description: &str) -> Self {
    Self { value: napi::symbol_for(description) }
  }

  pub fn description(&self) -> Option<String> {
    let key = napi::create_string_utf8("description");
    let value = napi::get_named_property(self.value, key);
    // TODO: how does the Node API handles symbols with undefined description?
    let property = NapiValues::from_napi_value(value);
    match property {
      NapiValues::String(string) => Some(string.into()),
      NapiValues::Null(_) => None,
      NapiValues::Undefined(_) => None,
      _ => panic!("Unsupported symbol description {:?}", property),
    }
  }
}
