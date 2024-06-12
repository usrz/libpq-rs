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
      .field("@", &self.napi_handle().value())
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

impl <'a> Into<NapiOk> for NapiValue<'a> {
  fn into(self) -> NapiOk {
    self.napi_handle().into()
  }
}

impl <'a> Into<NapiErr> for NapiValue<'a> {
  fn into(self) -> NapiErr {
    self.napi_handle().into()
  }
}

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
      napi::TypeOf::Bigint => Self::Bigint(handle),
      napi::TypeOf::Boolean => Self::Boolean(handle),
      napi::TypeOf::External => Self::External(handle),
      napi::TypeOf::Function => Self::Function(handle),
      napi::TypeOf::Null => Self::Null(handle),
      napi::TypeOf::Number => Self::Number(handle),
      napi::TypeOf::Object => Self::Object(handle),
      napi::TypeOf::String => Self::String(handle),
      napi::TypeOf::Symbol => Self::Symbol(handle),
      napi::TypeOf::Undefined => Self::Undefined(handle),
    }
  }
}
