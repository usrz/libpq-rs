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

pub trait NapiValue: TryFrom<napi::Value> + Into<NapiResult<NapiValues>> + Clone {
  unsafe fn as_napi_value(&self) -> napi::Value;
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

impl From<napi::Value> for NapiValues {
  fn from(value: napi::Value) -> Self {
    let value_type = napi::type_of(value);
    match value_type {
      nodejs_sys::napi_valuetype::napi_bigint => NapiValues::Bigint(NapiBigint { value }),
      nodejs_sys::napi_valuetype::napi_boolean => NapiValues::Boolean(NapiBoolean { value }),
      nodejs_sys::napi_valuetype::napi_external => todo!(),
      nodejs_sys::napi_valuetype::napi_function => NapiValues::Function(NapiFunction { value }),
      nodejs_sys::napi_valuetype::napi_null => NapiValues::Null(NapiNull { value }),
      nodejs_sys::napi_valuetype::napi_number => NapiValues::Number(NapiNumber { value }),
      nodejs_sys::napi_valuetype::napi_object => NapiValues::Object(NapiObject { value }),
      nodejs_sys::napi_valuetype::napi_string => NapiValues::String(NapiString { value }),
      nodejs_sys::napi_valuetype::napi_symbol => NapiValues::Symbol(NapiSymbol { value }),
      nodejs_sys::napi_valuetype::napi_undefined => NapiValues::Undefined(NapiUndefined { value }),
      #[allow(unreachable_patterns)] // this should *really* never happen...
      _ => panic!("Unsupported JavaScript type \"{:?}\"", value_type)
    }
  }
}

impl NapiValue for NapiValues {
  unsafe fn as_napi_value(&self) -> napi::Value {
    match self {
      NapiValues::Bigint(value) => value.value,
      NapiValues::Boolean(value) => value.value,
      NapiValues::Function(value) => value.value,
      NapiValues::Null(value) => value.value,
      NapiValues::Number(value) => value.value,
      NapiValues::Object(value) => value.value,
      NapiValues::String(value) => value.value,
      NapiValues::Symbol(value) => value.value,
      NapiValues::Undefined(value) => value.value,
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

    println!("SELF {:?} {}", TypeId::of::<Self>(), type_name::<Self>());
    println!("   T {:?} {}", TypeId::of::<T>(), type_name::<T>());
    println!("  BI {:?} {}", TypeId::of::<NapiBigint>(), type_name::<NapiBigint>());

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
      Some(downcasted) => Ok(downcasted.to_owned()),
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
    let value = unsafe { value.as_napi_value() };
    let this = unsafe { self.as_napi_value() };
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

pub(super) fn expect_type(value: napi::Value, expected: napi::ValueType) -> NapiResult<napi::Value> {
  let actual = napi::type_of(value);

  match actual == expected {
    false => Err(NapiError::from(format!("Type \"{:?}\" is not the expected \"{:?}\" type", actual, expected))),
    true => Ok(value),
  }
}
