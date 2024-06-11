use crate::napi;
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
use crate::NapiErr;

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

pub trait NapiType<'a>: Sized {
  fn napi_handle(&self) -> napi::Handle<'a>;
}
