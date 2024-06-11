use crate::napi;
use crate::types::*;
use crate::errors::*;
use std::fmt;
use std::any::type_name;

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

impl <'a> NapiType<'a> for NapiValue<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiValue<'a> {
  fn from_napi_handle(handle: napi::Handle<'a>) -> Result<Self, NapiErr> {
    let value_type = handle.type_of();
    match value_type {
      nodejs_sys::napi_valuetype::napi_bigint => Ok(Self::Bigint(handle)),
      nodejs_sys::napi_valuetype::napi_boolean => Ok(Self::Boolean(handle)),
      nodejs_sys::napi_valuetype::napi_external => Ok(Self::External(handle)),
      nodejs_sys::napi_valuetype::napi_function => Ok(Self::Function(handle)),
      nodejs_sys::napi_valuetype::napi_null => Ok(Self::Null(handle)),
      nodejs_sys::napi_valuetype::napi_number => Ok(Self::Number(handle)),
      nodejs_sys::napi_valuetype::napi_object => Ok(Self::Object(handle)),
      nodejs_sys::napi_valuetype::napi_string => Ok(Self::String(handle)),
      nodejs_sys::napi_valuetype::napi_symbol => Ok(Self::Symbol(handle)),
      nodejs_sys::napi_valuetype::napi_undefined => Ok(Self::Undefined(handle)),
      #[allow(unreachable_patterns)] // this should *really* never happen...
      _ => Err(format!("Unsupported JavaScript type \"{:?}\"", value_type).into())
    }
  }

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
// napi_type_from_value!(NapiExternal, External);
// napi_type_from_value!(NapiFunction, Function);
napi_type_from_value!(NapiNull, Null);
napi_type_from_value!(NapiNumber, Number);
napi_type_from_value!(NapiObject, Object);
napi_type_from_value!(NapiString, String);
napi_type_from_value!(NapiSymbol, Symbol);
napi_type_from_value!(NapiUndefined, Undefined);

// External has its own rule for generics...
impl <'a, T: 'static> From<NapiExternal<'a, T>> for NapiValue<'a> {
  fn from (value: NapiExternal<'a, T>) -> Self {
    Self::External(value.napi_handle())
  }
}


// ===== TRY_FROM NAPIVALUE -> NAPITYPE ========================================

macro_rules! napi_value_try_from_type {
  ($type:ident, $struct:ident) => {
    impl <'a> TryFrom<NapiValue<'a>> for $struct<'a> {
      type Error = NapiErr;

      fn try_from(value: NapiValue<'a>) -> Result<Self, Self::Error> {
        match value {
          NapiValue::$type(handle) => Ok($struct::from_napi_handle_unchecked(handle)),
          _ => Err(format!("Can't downcast {} into {:?}", value, type_name::<$struct>()).into()),
        }
      }
    }
  };
}

napi_value_try_from_type!(Bigint, NapiBigint);
napi_value_try_from_type!(Boolean, NapiBoolean);
// napi_value_try_from_type!(External, NapiExternal);
// napi_value_try_from_type!(Function, NapiFunction);
napi_value_try_from_type!(Null, NapiNull);
napi_value_try_from_type!(Number, NapiNumber);
napi_value_try_from_type!(Object, NapiObject);
napi_value_try_from_type!(String, NapiString);
napi_value_try_from_type!(Symbol, NapiSymbol);
napi_value_try_from_type!(Undefined, NapiUndefined);

// External has its own rules...
impl <'a, T: 'static> TryInto<NapiExternal<'a, T>> for NapiValue<'a> {
  type Error = NapiErr;

  fn try_into(self) -> Result<NapiExternal<'a, T>, Self::Error> {
    match self {
      Self::External(handle) => Ok(NapiExternal::<T>::from_napi_handle(handle)?),
      _ => Err(format!("Can't downcast {} into {:?}", self, type_name::<NapiExternal<T>>()).into()),
    }
  }
}
