use crate::napi;
use crate::types::*;

#[derive(Clone,Debug)]
pub struct NapiSymbol {
  reference: NapiReference,
}

impl NapiShape for NapiSymbol {}

impl NapiShapeInternal for NapiSymbol {
  fn as_napi_value(self) -> napi::Value {
    self.reference.value()
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { reference: value.into() }
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiSymbol {
  pub fn new(description: &str) -> Self {
    let value = napi::create_string_utf8(description);
    Self::from_napi_value(napi::create_symbol(value))
  }

  pub fn symbol_for(description: &str) -> Self {
    Self::from_napi_value(napi::symbol_for(description))
  }

  pub fn description(&self) -> Option<String> {
    let key = napi::create_string_utf8("description");
    let value = napi::get_property(self.reference.value(), key);

    let property = NapiValue::from_napi_value(value);
    match property {
      NapiValue::String(string) => Some(string.into()),
      NapiValue::Undefined(_) => None,
      NapiValue::Null(_) => None,
      _ => panic!("Unsupported symbol description {:?}", property),
    }
  }
}
