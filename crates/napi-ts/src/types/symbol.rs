use crate::napi;
use crate::types::*;

#[derive(Clone)]
pub struct NapiSymbol {
  reference: NapiReference,
}

impl Debug for NapiSymbol {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiExternal")
      .field("@", &self.reference.handle())
      .finish()
  }
}

impl NapiShape for NapiSymbol {}

impl NapiShapeInternal for NapiSymbol {
  fn into_napi_value(self) -> napi::Handle {
    self.reference.handle()
  }

  fn from_napi_value(handle: napi::Handle) -> Self {
    Self { reference: handle.into() }
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiSymbol {
  pub fn new(description: &str) -> Self {
    let handle = napi::create_string_utf8(description);
    Self::from_napi_value(napi::create_symbol(handle))
  }

  pub fn symbol_for(description: &str) -> Self {
    Self::from_napi_value(napi::symbol_for(description))
  }

  pub fn description(&self) -> Option<String> {
    let key = napi::create_string_utf8("description");
    let value = napi::get_property(self.reference.handle(), key);

    let property = NapiValue::from(value);
    match property {
      NapiValue::String(string) => Some(string.into()),
      NapiValue::Undefined(_) => None,
      NapiValue::Null(_) => None,
      _ => panic!("Unsupported symbol description {:?}", property),
    }
  }
}
