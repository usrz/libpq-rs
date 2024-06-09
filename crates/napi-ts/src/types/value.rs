use crate::errors::NapiResult;
use crate::napi;
use crate::types::*;

use std::any::type_name;
use std::any::Any;

// ========================================================================== //
// VALUE ENUM (ALL TYPES)                                                     //
// ========================================================================== //

#[derive(Debug)]
pub enum NapiValue<'a> {
  Bigint(NapiBigint<'a>),
  Boolean(NapiBoolean<'a>),
  // External(NapiExternalRef),
  // Function(NapiFunction),
  Null(NapiNull<'a>),
  Number(NapiNumber<'a>),
  Object(NapiObject<'a>),
  String(NapiString<'a>),
  Symbol(NapiSymbol<'a>),
  Undefined(NapiUndefined<'a>),
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl NapiType for NapiValue<'_> {}

impl NapiFrom<napi::Handle> for NapiValue<'_> {
  fn napi_from(handle: napi::Handle, env: napi::Env) -> Self {
    let value_type = napi::type_of(handle);
    match value_type {
      nodejs_sys::napi_valuetype::napi_bigint => NapiValue::Bigint(NapiBigint::napi_from(handle, env)),
      nodejs_sys::napi_valuetype::napi_boolean => NapiValue::Boolean(NapiBoolean::napi_from(handle, env)),
      // nodejs_sys::napi_valuetype::napi_external => NapiValue::External(NapiExternalRef::napi_from(handle, env)),
      // nodejs_sys::napi_valuetype::napi_function => NapiValue::Function(NapiFunction::napi_from(handle, env)),
      nodejs_sys::napi_valuetype::napi_null => NapiValue::Null(NapiNull::napi_from(handle, env)),
      nodejs_sys::napi_valuetype::napi_number => NapiValue::Number(NapiNumber::napi_from(handle, env)),
      nodejs_sys::napi_valuetype::napi_object => NapiValue::Object(NapiObject::napi_from(handle, env)),
      nodejs_sys::napi_valuetype::napi_string => NapiValue::String(NapiString::napi_from(handle, env)),
      nodejs_sys::napi_valuetype::napi_symbol => NapiValue::Symbol(NapiSymbol::napi_from(handle, env)),
      nodejs_sys::napi_valuetype::napi_undefined => NapiValue::Undefined(NapiUndefined::napi_from(handle, env)),
      #[allow(unreachable_patterns)] // this should *really* never happen...
      _ => panic!("Unsupported JavaScript type \"{:?}\"", value_type)
    }
  }
}

impl NapiInto<napi::Handle> for NapiValue<'_> {
  fn napi_into(self, env: napi::Env) -> napi::Handle {
    match self {
      NapiValue::Bigint(value) => value.napi_into(env),
      NapiValue::Boolean(value) => value.napi_into(env),
      // NapiValue::External(value) => value.napi_into(env),
      // NapiValue::Function(value) => value.napi_into(env),
      NapiValue::Null(value) => value.napi_into(env),
      NapiValue::Number(value) => value.napi_into(env),
      NapiValue::Object(value) => value.napi_into(env),
      NapiValue::String(value) => value.napi_into(env),
      NapiValue::Symbol(value) => value.napi_into(env),
      NapiValue::Undefined(value) => value.napi_into(env),
    }
  }
}


// unsafe impl Send for NapiValue {}

// impl <T: NapiShape> From<T> for NapiValue {
//   fn from(value: T) -> Self {
//     Self::from(value.into_napi_value())
//   }
// }

// impl Into<napi::Handle> for NapiValue {
//   fn into(self) -> napi::Handle {
//     match self {
//       // NapiValue::Bigint(value) => value.into_napi_value(),
//       // NapiValue::Boolean(value) => value.into_napi_value(),
//       NapiValue::External(value) => value.into_napi_value(),
//       NapiValue::Function(value) => value.into_napi_value(),
//       NapiValue::Null(value) => value.into_napi_value(),
//       NapiValue::Number(value) => value.into_napi_value(),
//       NapiValue::Object(value) => value.into_napi_value(),
//       NapiValue::String(value) => value.into_napi_value(),
//       NapiValue::Symbol(value) => value.into_napi_value(),
//       NapiValue::Undefined(value) => value.into_napi_value(),
//     }
//   }
// }

// impl NapiShapeInternal for NapiValue {
//   fn into_napi_value(self) -> napi::Handle {
//     self.into()
//   }

//   fn from_napi_value(value: napi::Handle) -> Self {
//     Self::from(value)
//   }
// }

// impl NapiValue {
//   pub fn downcast<T: NapiShape + 'static>(&self) -> NapiResult<T> {
//     let result = match self {
//       // NapiValue::Bigint(value) => (value as &dyn Any).downcast_ref::<T>(),
//       // NapiValue::Boolean(value) => (value as &dyn Any).downcast_ref::<T>(),
//       NapiValue::External(value) => return unsafe { value.downcast::<T>() },
//       NapiValue::Function(value) => (value as &dyn Any).downcast_ref::<T>(),
//       NapiValue::Null(value) => (value as &dyn Any).downcast_ref::<T>(),
//       NapiValue::Number(value) => (value as &dyn Any).downcast_ref::<T>(),
//       NapiValue::Object(value) => (value as &dyn Any).downcast_ref::<T>(),
//       NapiValue::String(value) => (value as &dyn Any).downcast_ref::<T>(),
//       NapiValue::Symbol(value) => (value as &dyn Any).downcast_ref::<T>(),
//       NapiValue::Undefined(value) => (value as &dyn Any).downcast_ref::<T>(),
//     };

//     if let Some(downcasted) = result {
//       return Ok(downcasted.clone())
//     }

//     // No way to downcast our value...
//     let from = match self {
//       // NapiValue::Bigint(_) => "NapiBigint",
//       // NapiValue::Boolean(_) => "NapiBoolean",
//       NapiValue::External(_) => "NapiExternal",
//       NapiValue::Function(_) => "NapiFunction",
//       NapiValue::Null(_) => "NapiNull",
//       NapiValue::Number(_) => "NapiNumber",
//       NapiValue::Object(_) => "NapiObject",
//       NapiValue::String(_) => "NapiString",
//       NapiValue::Symbol(_) => "NapiSymbol",
//       NapiValue::Undefined(_) => "NapiUndefined",
//     };
//     let into = type_name::<T>();
//     Err(format!("Unable to downcast \"{}\" into \"{}\"", from, into).into())
//   }
// }

// // ========================================================================== //
// // PROPERTIES                                                                 //
// // ========================================================================== //

// pub trait NapiValueWithProperties: NapiShape {
//   fn get_property(&self, key: &str) -> Option<NapiValue> {
//     let key = napi::create_string_utf8(key);
//     let this = self.clone().into_napi_value();
//     let result = napi::get_property(this, key);
//     let value = NapiValue::from(result);

//     match value {
//       NapiValue::Undefined(_) => None,
//       value => Some(value),
//     }
//   }

//   fn set_property(&self, key: &str, value: &impl NapiShape) -> &Self {
//     let key = napi::create_string_utf8(key);
//     let value = value.clone().into_napi_value();
//     let this = self.clone().into_napi_value();
//     napi::set_property(this, key, value);
//     self
//   }

//   // fn set_property_bool(&self, key: &str, value: bool) -> &Self {
//   //   self.set_property(key, &NapiBoolean::from(value))
//   // }

//   fn set_property_null(&self, key: &str) -> &Self {
//     self.set_property(key, &NapiNull::new())
//   }

//   fn set_property_int(&self, key: &str, value: i32) -> &Self {
//     self.set_property(key, &NapiNumber::from(value))
//   }

//   fn set_property_float(&self, key: &str, value: f64) -> &Self {
//     self.set_property(key, &NapiNumber::from(value))
//   }

//   fn set_property_str(&self, key: &str, value: &str) -> &Self {
//     self.set_property(key, &NapiString::from(value))
//   }

//   fn set_property_string(&self, key: &str, value: String) -> &Self {
//     self.set_property(key, &NapiString::from(value))
//   }

//   fn set_property_undefined(&self, key: &str) -> &Self {
//     self.set_property(key, &NapiUndefined::new())
//   }
// }
