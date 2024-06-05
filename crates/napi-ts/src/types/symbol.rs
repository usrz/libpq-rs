use crate::napi;
use crate::types::*;
use crate::napi::create_reference;

#[derive(Debug)]
pub struct NapiSymbol {
  value: napi::Value,
  reference: napi::Reference,
}

impl NapiShape for NapiSymbol {}

impl Clone for NapiSymbol {
  fn clone(&self) -> Self {
    napi::reference_ref(self.reference);
    Self { value: self.value, reference: self.reference }
  }
}

impl Drop for NapiSymbol {
  fn drop(&mut self) {
    napi::reference_unref(self.reference);
  }
}

impl NapiShapeInternal for NapiSymbol {
  fn as_napi_value(&self) -> napi::Value {
    self.value
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { value, reference: create_reference(value, 1) }
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
    let value = napi::get_named_property(self.value, key);
    // TODO: how does the Node API handles symbols with undefined description?
    let property = NapiValue::from_napi_value(value);
    match property {
      NapiValue::String(string) => Some(string.into()),
      NapiValue::Null(_) => None,
      NapiValue::Undefined(_) => None,
      _ => panic!("Unsupported symbol description {:?}", property),
    }
  }
}
