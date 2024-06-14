use crate::types::*;
use std::fmt;

// ========================================================================== //
// VALUE ENUM (ALL TYPES)                                                     //
// ========================================================================== //

#[derive(Clone, Copy)]
pub enum NapiValue {
  Bigint(napi::Handle),
  Boolean(napi::Handle),
  External(napi::Handle),
  Function(napi::Handle),
  Null(napi::Handle),
  Number(napi::Handle),
  Object(napi::Handle),
  String(napi::Handle),
  Symbol(napi::Handle),
  Undefined(napi::Handle),
}

// ===== DEBUG / DISPLAY NICETIES ==============================================

impl fmt::Debug for NapiValue {
  fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
    fm.debug_tuple(&format!("{}", self))
      .field(&self.napi_handle())
      .finish()
  }
}

impl fmt::Display for NapiValue {
  fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
    fm.write_str(match self {
      Self::Bigint(_) => "NapiValue<Bigint>",
      Self::Boolean(_) => "NapiValue<Boolean>",
      Self::External(_) => "NapiValue<External>",
      Self::Function(_) => "NapiValue<Function>",
      Self::Null(_) => "NapiValue<Null>",
      Self::Number(_) => "NapiValue<Number>",
      Self::Object(_) => "NapiValue<Object>",
      Self::String(_) => "NapiValue<String>",
      Self::Symbol(_) => "NapiValue<Symbol>",
      Self::Undefined(_) => "NapiValue<Undefined>",
    })
  }
}

// ===== NAPI TYPE BASICS ======================================================

impl NapiType for NapiValue {
  fn into_napi_value(self) -> NapiValue {
    self // identity....
  }
  fn try_from_napi_value(value: &NapiValue) -> Result<Self, NapiErr> {
    Ok(*value) // identity
  }
}

impl NapiTypeInternal for NapiValue {
  fn from_handle(handle: napi::Handle) -> Self {
    let value_type = handle.type_of();

    match value_type {
      napi::TypeOf::Bigint => NapiValue::Bigint(handle).into(),
      napi::TypeOf::Boolean => NapiValue::Boolean(handle).into(),
      napi::TypeOf::External => NapiValue::External(handle).into(),
      napi::TypeOf::Function => NapiValue::Function(handle).into(),
      napi::TypeOf::Null => NapiValue::Null(handle).into(),
      napi::TypeOf::Number => NapiValue::Number(handle).into(),
      napi::TypeOf::Object => NapiValue::Object(handle).into(),
      napi::TypeOf::String => NapiValue::String(handle).into(),
      napi::TypeOf::Symbol => NapiValue::Symbol(handle).into(),
      napi::TypeOf::Undefined => NapiValue::Undefined(handle).into(),
    }
  }

  fn napi_handle(&self) -> napi::Handle {
    match self {
      Self::Bigint(handle) => *handle,
      Self::Boolean(handle) => *handle,
      Self::External(handle) => *handle,
      Self::Function(handle) => *handle,
      Self::Null(handle) => *handle,
      Self::Number(handle) => *handle,
      Self::Object(handle) => *handle,
      Self::String(handle) => *handle,
      Self::Symbol(handle) => *handle,
      Self::Undefined(handle) => *handle,
    }
  }
}
