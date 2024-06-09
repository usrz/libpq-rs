use crate::napi;
use std::fmt::Debug;

mod bigint;
mod boolean;
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
pub use null::*;
pub use number::*;
pub use object::*;
pub use properties::*;
pub use string::*;
pub use symbol::*;
pub use undefined::*;
pub use value::*;

pub(crate) trait NapiFrom<T>: Sized {
  fn napi_from(value: T, env: napi::Env) -> Self;
}

pub(crate) trait NapiInto<T>: Sized {
  fn napi_into(self, env: napi::Env) -> T;
}

impl<T, U> NapiInto<U> for T
where
  U: NapiFrom<T>,
{
  fn napi_into(self, env: napi::Env) -> U {
    U::napi_from(self, env)
  }
}

pub (crate) trait NapiTypeInternal<'a> {
  fn from_napi(env: napi::Env, handle: napi::Handle) -> Self;
  fn napi_handle(&self) -> napi::Handle;
}

#[allow(private_bounds)]
pub trait NapiType<'a>: Sized + NapiTypeInternal<'a> {}
