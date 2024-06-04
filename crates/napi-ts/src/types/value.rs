use crate::napi;
use crate::errors::*;
use super::NapiString;
use super::NapiNull;
use super::NapiBoolean;
use super::NapiUndefined;
use super::NapiObject;
use super::NapiNumber;
use super::NapiBigint;

pub trait NapiValue: TryFrom<napi::Value> + Into<NapiResult<NapiValues>> {
  unsafe fn as_napi_value(&self) -> napi::Value;
}

pub enum NapiValues {
  Bigint(NapiBigint),
  Boolean(NapiBoolean),
  Null(NapiNull),
  Number(NapiNumber),
  Object(NapiObject),
  String(NapiString),
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

impl From<NapiUndefined> for NapiValues {
  fn from(value: NapiUndefined) -> Self {
    NapiValues::Undefined(value)
  }
}

impl TryFrom<napi::Value> for NapiValues {
  type Error = NapiError;

  fn try_from(value: napi::Value) -> NapiResult<Self> {
    let value_type = napi::type_of(value);
    match value_type {
      nodejs_sys::napi_valuetype::napi_bigint => Ok(NapiValues::Bigint(NapiBigint { value })),
      nodejs_sys::napi_valuetype::napi_boolean => Ok(NapiValues::Boolean(NapiBoolean { value })),
      nodejs_sys::napi_valuetype::napi_external => todo!(),
      nodejs_sys::napi_valuetype::napi_function => todo!(),
      nodejs_sys::napi_valuetype::napi_null => Ok(NapiValues::Null(NapiNull { value })),
      nodejs_sys::napi_valuetype::napi_number => Ok(NapiValues::Number(NapiNumber { value })),
      nodejs_sys::napi_valuetype::napi_object => Ok(NapiValues::Object(NapiObject { value })),
      nodejs_sys::napi_valuetype::napi_string => Ok(NapiValues::String(NapiString { value })),
      nodejs_sys::napi_valuetype::napi_symbol => todo!(),
      nodejs_sys::napi_valuetype::napi_undefined => Ok(NapiValues::Undefined(NapiUndefined { value })),
      #[allow(unreachable_patterns)]
      _ => Err(NapiError::from(format!("Unsupported JavaScript type \"{:?}\"", value_type)))
    }
  }
}

impl NapiValue for NapiValues {
  unsafe fn as_napi_value(&self) -> napi::Value {
    match self {
      NapiValues::Bigint(value) => value.value,
      NapiValues::Boolean(value) => value.value,
      NapiValues::Null(value) => value.value,
      NapiValues::Number(value) => value.value,
      NapiValues::Object(value) => value.value,
      NapiValues::String(value) => value.value,
      NapiValues::Undefined(value) => value.value,
    }
  }
}

impl Into<NapiResult<NapiValues>> for NapiValues {
  fn into(self) -> NapiResult<NapiValues> {
    Ok(self)
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
