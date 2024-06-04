use crate::errors::*;
use crate::napi;
use crate::types::*;

use std::any::type_name;
use std::any::Any;
use std::fmt::Debug;

// ========================================================================== //
// TRAITS                                                                     //
// ========================================================================== //

pub(crate) trait NapiShapeInternal: Clone + Debug {
  fn as_napi_value(&self) -> napi::Value;
  fn from_napi_value(value: napi::Value) -> Self;
}

#[allow(private_bounds)]
pub trait NapiShape: NapiShapeInternal {
  fn ok(self) -> NapiResult<NapiReturn> {
    Ok(NapiReturn::from_napi_value(self.as_napi_value()))
  }
}

// ========================================================================== //
// RETURN VALUE                                                               //
// ========================================================================== //

#[derive(Clone,Debug)]
pub struct NapiReturn {
  value: napi::Value
}

impl NapiShapeInternal for NapiReturn {
  fn as_napi_value(&self) -> napi::Value {
    self.value
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { value }
  }
}

impl <T: NapiShape> From<T> for NapiReturn {
  fn from(value: T) -> Self {
    Self::from_napi_value(value.as_napi_value())
  }
}

impl NapiReturn {
  pub fn void() -> NapiResult<NapiReturn> {
    Ok(Self { value: napi::get_undefined() })
  }
}

// ========================================================================== //
// VALUE ENUM (ALL TYPES)                                                     //
// ========================================================================== //

#[derive(Clone,Debug)]
pub enum NapiValue {
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

impl <T: NapiShape> From<T> for NapiValue {
  fn from(value: T) -> Self {
    Self::from_napi_value(value.as_napi_value())
  }
}

impl NapiShapeInternal for NapiValue {
  fn as_napi_value(&self) -> napi::Value {
    match self {
      NapiValue::Bigint(value) => value.as_napi_value(),
      NapiValue::Boolean(value) => value.as_napi_value(),
      NapiValue::Function(value) => value.as_napi_value(),
      NapiValue::Null(value) => value.as_napi_value(),
      NapiValue::Number(value) => value.as_napi_value(),
      NapiValue::Object(value) => value.as_napi_value(),
      NapiValue::String(value) => value.as_napi_value(),
      NapiValue::Symbol(value) => value.as_napi_value(),
      NapiValue::Undefined(value) => value.as_napi_value(),
    }
  }

  fn from_napi_value(value: napi::Value) -> Self {
    let value_type = napi::type_of(value);
    match value_type {
      nodejs_sys::napi_valuetype::napi_bigint => NapiValue::Bigint(NapiBigint::from_napi_value(value)),
      nodejs_sys::napi_valuetype::napi_boolean => NapiValue::Boolean(NapiBoolean::from_napi_value(value)),
      nodejs_sys::napi_valuetype::napi_external => todo!(),
      nodejs_sys::napi_valuetype::napi_function => NapiValue::Function(NapiFunction::from_napi_value(value)),
      nodejs_sys::napi_valuetype::napi_null => NapiValue::Null(NapiNull::from_napi_value(value)),
      nodejs_sys::napi_valuetype::napi_number => NapiValue::Number(NapiNumber::from_napi_value(value)),
      nodejs_sys::napi_valuetype::napi_object => NapiValue::Object(NapiObject::from_napi_value(value)),
      nodejs_sys::napi_valuetype::napi_string => NapiValue::String(NapiString::from_napi_value(value)),
      nodejs_sys::napi_valuetype::napi_symbol => NapiValue::Symbol(NapiSymbol::from_napi_value(value)),
      nodejs_sys::napi_valuetype::napi_undefined => NapiValue::Undefined(NapiUndefined::from_napi_value(value)),
      #[allow(unreachable_patterns)] // this should *really* never happen...
      _ => panic!("Unsupported JavaScript type \"{:?}\"", value_type)
    }
  }
}

impl NapiValue {
  pub fn downcast<T: NapiShape + 'static>(&self) -> NapiResult<T> {
    let result = match self {
      NapiValue::Bigint(value) => (value as &dyn Any).downcast_ref::<T>(),
      NapiValue::Boolean(value) => (value as &dyn Any).downcast_ref::<T>(),
      NapiValue::Function(value) => (value as &dyn Any).downcast_ref::<T>(),
      NapiValue::Null(value) => (value as &dyn Any).downcast_ref::<T>(),
      NapiValue::Number(value) => (value as &dyn Any).downcast_ref::<T>(),
      NapiValue::Object(value) => (value as &dyn Any).downcast_ref::<T>(),
      NapiValue::String(value) => (value as &dyn Any).downcast_ref::<T>(),
      NapiValue::Symbol(value) => (value as &dyn Any).downcast_ref::<T>(),
      NapiValue::Undefined(value) => (value as &dyn Any).downcast_ref::<T>(),
    };

    match result {
      Some(downcasted) => Ok(downcasted.clone()),
      None => {
        let from = match self {
          NapiValue::Bigint(_) => "NapiBigint",
          NapiValue::Boolean(_) => "NapiBoolean",
          NapiValue::Function(_) => "NapiFunction",
          NapiValue::Null(_) => "NapiNull",
          NapiValue::Number(_) => "NapiNumber",
          NapiValue::Object(_) => "NapiObject",
          NapiValue::String(_) => "NapiString",
          NapiValue::Symbol(_) => "NapiSymbol",
          NapiValue::Undefined(_) => "NapiUndefined",
        };
        let into = type_name::<T>().rsplit_once(":").unwrap().1;
        Err(format!("Unable to downcast \"{}\" into \"{}\"", from, into).into())
      }
    }
  }
}


pub trait NapiValueWithProperties: NapiShape {
  fn set_property(&self, key: &str, value: &impl NapiShape) -> &Self {
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
