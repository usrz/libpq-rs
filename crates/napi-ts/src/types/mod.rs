use crate::napi;
use crate::errors::*;
use std::fmt::Debug;

mod bigint;
mod boolean;
mod external;
mod null;
mod number;
mod object;
mod properties;
mod string;
mod symbol;
mod undefined;
mod value;

pub use bigint::*;
pub use boolean::*;
pub use external::*;
pub use null::*;
pub use number::*;
pub use object::*;
pub use properties::*;
pub use string::*;
pub use symbol::*;
pub use undefined::*;
pub use value::*;

// ===== CONVERSION ============================================================

pub (crate) trait NapiFrom<'a, T>: Sized {
  fn napi_from(value: T, env: napi::Env<'a>) -> Self;
}

pub (crate) trait NapiInto<'a, T>: Sized {
  fn napi_into(self, env: napi::Env<'a>) -> T;
}

impl<'a, T, U> NapiInto<'a, U> for T
where
  U: NapiFrom<'a, T>,
{
  fn napi_into(self, env: napi::Env<'a>) -> U {
    U::napi_from(self, env)
  }
}

// ===== TYPES =================================================================

pub trait NapiType<'a>: Into<NapiOk> + Into<NapiErr> + Sized {
  fn napi_handle(&self) -> napi::Handle<'a>;
}

macro_rules! napi_type {
  (
    $type:ident // The final type, e.g. NapiObject
    $(< $($params:ident),+ >)?, // Any generic parameters without lifetime
    $value:ident // The NapiValue type to associate with this
  ) => {
    impl <'a $(, $($params: 'static)?)?> NapiType<'a> for $type<'a $(, $($params)?)?> {
      fn napi_handle(&self) -> napi::Handle<'a> {
        self.handle
      }
    }

    impl <'a $(, $($params: 'static)?)?> Into<NapiErr> for $type<'a $(, $($params)?)?> {
      fn into(self) -> NapiErr {
        self.napi_handle().into()
      }
    }

    impl <'a $(, $($params: 'static)?)?> Into<NapiOk> for $type<'a $(, $($params)?)?> {
      fn into(self) -> NapiOk {
        self.napi_handle().into()
      }
    }

    impl <'a $(, $($params: 'static)?)?> Into<NapiValue<'a>> for $type<'a $(, $($params)?)?> {
      fn into (self) -> NapiValue<'a> {
        NapiValue::$value(self.napi_handle())
      }
    }
  };
}

pub (self) use napi_type;
