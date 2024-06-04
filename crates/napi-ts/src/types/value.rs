use crate::errors::*;
use crate::napi;
use super::NapiBigint;
use super::NapiBoolean;
use super::NapiFunction;
use super::NapiNull;
use super::NapiNumber;
use super::NapiObject;
use super::NapiString;
use super::NapiSymbol;
use super::NapiUndefined;
use std::any::TypeId;
use std::any::type_name;
use std::any::Any;

pub(crate) trait NapiValueInternal: Into<NapiResult<NapiValues>> + Clone {
  fn as_napi_value(&self) -> napi::Value;
  fn from_napi_value(value: napi::Value) -> Self;
}

#[allow(private_bounds)]
pub trait NapiValue: NapiValueInternal {
  // Public marker of "NapiValueInternal"
}

#[derive(Clone,Debug)]
pub enum NapiValues {
  Bigint(NapiBigint),
  Boolean(NapiBoolean),
  Function(NapiFunction),
  Null(NapiNull),
  Number(NapiNumber),
  Object(NapiObject),
  String(NapiString),
  Symbol(NapiSymbol),
  Undefined(NapiUndefined),
}

impl From<NapiBigint> for NapiValues {
  fn from(value: NapiBigint) -> Self {
    NapiValues::Bigint(value)
  }
}

impl From<NapiBoolean> for NapiValues {
  fn from(value: NapiBoolean) -> Self {
    NapiValues::Boolean(value)
  }
}

impl From<NapiFunction> for NapiValues {
  fn from(value: NapiFunction) -> Self {
    NapiValues::Function(value)
  }
}

impl From<NapiNull> for NapiValues {
  fn from(value: NapiNull) -> Self {
    NapiValues::Null(value)
  }
}

impl From<NapiNumber> for NapiValues {
  fn from(value: NapiNumber) -> Self {
    NapiValues::Number(value)
  }
}

impl From<NapiObject> for NapiValues {
  fn from(value: NapiObject) -> Self {
    NapiValues::Object(value)
  }
}

impl From<NapiString> for NapiValues {
  fn from(value: NapiString) -> Self {
    NapiValues::String(value)
  }
}

impl From<NapiSymbol> for NapiValues {
  fn from(value: NapiSymbol) -> Self {
    NapiValues::Symbol(value)
  }
}

impl From<NapiUndefined> for NapiValues {
  fn from(value: NapiUndefined) -> Self {
    NapiValues::Undefined(value)
  }
}

impl NapiValue for NapiValues {}

impl NapiValueInternal for NapiValues {
  fn as_napi_value(&self) -> napi::Value {
    match self {
      NapiValues::Bigint(value) => value.as_napi_value(),
      NapiValues::Boolean(value) => value.as_napi_value(),
      NapiValues::Function(value) => value.as_napi_value(),
      NapiValues::Null(value) => value.as_napi_value(),
      NapiValues::Number(value) => value.as_napi_value(),
      NapiValues::Object(value) => value.as_napi_value(),
      NapiValues::String(value) => value.as_napi_value(),
      NapiValues::Symbol(value) => value.as_napi_value(),
      NapiValues::Undefined(value) => value.as_napi_value(),
    }
  }

  fn from_napi_value(value: napi::Value) -> Self {
    let value_type = napi::type_of(value);
    match value_type {
      nodejs_sys::napi_valuetype::napi_bigint => NapiValues::Bigint(NapiBigint::from_napi_value(value)),
      nodejs_sys::napi_valuetype::napi_boolean => NapiValues::Boolean(NapiBoolean::from_napi_value(value)),
      nodejs_sys::napi_valuetype::napi_external => todo!(),
      nodejs_sys::napi_valuetype::napi_function => NapiValues::Function(NapiFunction::from_napi_value(value)),
      nodejs_sys::napi_valuetype::napi_null => NapiValues::Null(NapiNull::from_napi_value(value)),
      nodejs_sys::napi_valuetype::napi_number => NapiValues::Number(NapiNumber::from_napi_value(value)),
      nodejs_sys::napi_valuetype::napi_object => NapiValues::Object(NapiObject::from_napi_value(value)),
      nodejs_sys::napi_valuetype::napi_string => NapiValues::String(NapiString::from_napi_value(value)),
      nodejs_sys::napi_valuetype::napi_symbol => NapiValues::Symbol(NapiSymbol::from_napi_value(value)),
      nodejs_sys::napi_valuetype::napi_undefined => NapiValues::Undefined(NapiUndefined::from_napi_value(value)),
      #[allow(unreachable_patterns)] // this should *really* never happen...
      _ => panic!("Unsupported JavaScript type \"{:?}\"", value_type)
    }
  }
}

impl Into<NapiResult<NapiValues>> for NapiValues {
  fn into(self) -> NapiResult<NapiValues> {
    Ok(self)
  }
}

impl NapiValues {
  pub fn downcast<T: NapiValue + 'static>(&self) -> NapiResult<T> {
    let result = match self {
      NapiValues::Bigint(value) => (value as &dyn Any).downcast_ref::<T>(),
      NapiValues::Boolean(value) => (value as &dyn Any).downcast_ref::<T>(),
      NapiValues::Function(value) => (value as &dyn Any).downcast_ref::<T>(),
      NapiValues::Null(value) => (value as &dyn Any).downcast_ref::<T>(),
      NapiValues::Number(value) => (value as &dyn Any).downcast_ref::<T>(),
      NapiValues::Object(value) => (value as &dyn Any).downcast_ref::<T>(),
      NapiValues::String(value) => (value as &dyn Any).downcast_ref::<T>(),
      NapiValues::Symbol(value) => (value as &dyn Any).downcast_ref::<T>(),
      NapiValues::Undefined(value) => (value as &dyn Any).downcast_ref::<T>(),
    };

    match result {
      Some(downcasted) => Ok(downcasted.clone()),
      None => {
        let from = match self {
          NapiValues::Bigint(_) => "Bigint",
          NapiValues::Boolean(_) => "Boolean",
          NapiValues::Function(_) => "Function",
          NapiValues::Null(_) => "Null",
          NapiValues::Number(_) => "Number",
          NapiValues::Object(_) => "Object",
          NapiValues::String(_) => "String",
          NapiValues::Symbol(_) => "Symbol",
          NapiValues::Undefined(_) => "Undefined",
        };
        let into = type_name::<T>().rsplit_once(":").unwrap().1;
        Err(format!("Unable to downcast \"{}\" into \"{}\"", from, into).into())
      }
    }
  }
}


pub trait NapiValueWithProperties: NapiValue {

  fn set_property(&self, key: &str, value: &impl NapiValue) -> &Self {
    let key = napi::create_string_utf8(key);
    let value = value.as_napi_value();
    let this = self.as_napi_value();
    napi::set_named_property(this, key, value);
    self
  }

  fn set_property_bool(&self, key: &str, value: bool) -> &Self {
    self.set_property(key, &NapiBoolean::from(value))
  }

  fn set_property_null(&self, key: &str) -> &Self {
    self.set_property(key, &NapiNull::new())
  }

  fn set_property_int(&self, key: &str, value: i32) -> &Self {
    self.set_property(key, &NapiNumber::from(value))
  }

  fn set_property_float(&self, key: &str, value: f64) -> &Self {
    self.set_property(key, &NapiNumber::from(value))
  }

  fn set_property_str(&self, key: &str, value: &str) -> &Self {
    self.set_property(key, &NapiString::from(value))
  }

  fn set_property_string(&self, key: &str, value: String) -> &Self {
    self.set_property(key, &NapiString::from(value))
  }

  fn set_property_undefined(&self, key: &str) -> &Self {
    self.set_property(key, &NapiUndefined::new())
  }
}
