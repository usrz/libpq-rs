use crate::napi;
use crate::types::*;
use std::fmt;

// ========================================================================== //
// VALUE ENUM (ALL TYPES)                                                     //
// ========================================================================== //

pub enum NapiValue<'a> {
  Bigint(napi::Handle<'a>),
  Boolean(napi::Handle<'a>),
  External(napi::Handle<'a>),
  Function(napi::Handle<'a>),
  Null(napi::Handle<'a>),
  Number(napi::Handle<'a>),
  Object(napi::Handle<'a>),
  String(napi::Handle<'a>),
  Symbol(napi::Handle<'a>),
  Undefined(napi::Handle<'a>),
}

impl fmt::Debug for NapiValue<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct(&format!("{}", self))
      .field("@", &self.napi_handle())
      .finish()
  }
}

impl fmt::Display for NapiValue<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(match self {
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

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiValue<'a> {
  fn napi_handle(&self) -> napi::Handle<'a> {
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

impl <'a> From<napi::Handle<'a>> for NapiValue<'a> {
  fn from(handle: napi::Handle<'a>) -> Self {
    let value_type = handle.type_of();
    match value_type {
      nodejs_sys::napi_valuetype::napi_bigint => Self::Bigint(handle),
      nodejs_sys::napi_valuetype::napi_boolean => Self::Boolean(handle),
      nodejs_sys::napi_valuetype::napi_external => Self::External(handle),
      nodejs_sys::napi_valuetype::napi_function => Self::Function(handle),
      nodejs_sys::napi_valuetype::napi_null => Self::Null(handle),
      nodejs_sys::napi_valuetype::napi_number => Self::Number(handle),
      nodejs_sys::napi_valuetype::napi_object => Self::Object(handle),
      nodejs_sys::napi_valuetype::napi_string => Self::String(handle),
      nodejs_sys::napi_valuetype::napi_symbol => Self::Symbol(handle),
      nodejs_sys::napi_valuetype::napi_undefined => Self::Undefined(handle),
      #[allow(unreachable_patterns)] // this should *really* never happen...
      _ => panic!("Unsupported JavaScript type \"{:?}\"", value_type)
    }
  }
}

// ===== FROM NAPITYPE -> NAPIVALUE ============================================

macro_rules! napi_type_from_value {
  ($struct:ident, $type:ident) => {
    impl <'a> From<$struct<'a>> for NapiValue<'a> {
      fn from (value: $struct<'a>) -> Self {
        Self::$type(value.napi_handle())
      }
    }
  };
}

napi_type_from_value!(NapiBigint, Bigint);
napi_type_from_value!(NapiBoolean, Boolean);
// napi_type_from_value!(NapiFunction, Function);
napi_type_from_value!(NapiNull, Null);
napi_type_from_value!(NapiNumber, Number);
napi_type_from_value!(NapiObject, Object);
napi_type_from_value!(NapiString, String);
napi_type_from_value!(NapiSymbol, Symbol);
napi_type_from_value!(NapiUndefined, Undefined);

// External is generic, so I can't be fussed
impl <'a, T: 'static> From<NapiExternal<'a, T>> for NapiValue<'a> {
  fn from (value: NapiExternal<'a, T>) -> Self {
    Self::External(value.napi_handle())
  }
}
