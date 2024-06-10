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
use std::marker::PhantomData;
use crate::NapiErr;

// ===== CONVERSION ============================================================

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

// ===== HANDLES ===============================================================

pub struct NapiHandle<'a> {
  pub (crate) phantom: PhantomData<&'a mut ()>,
  pub (crate) handle: napi::Handle,
  pub (crate) env: napi::Env,
}

impl <'a> NapiHandle<'a> {
  pub (crate) fn from_napi(env: napi::Env, handle: napi::Handle) -> Self {
    Self { phantom: PhantomData, handle, env }
  }
}

// ===== TYPES =================================================================

pub (crate) trait NapiTypeInternal<'a>: Sized {
  fn get_napi_handle(&self) -> &NapiHandle<'a>;

  fn from_napi_handle(handle: NapiHandle<'a>) -> Result<Self, NapiErr>;

  fn from_napi_handle_unchecked(handle: NapiHandle<'a>) -> Self {
    Self::from_napi_handle(handle).unwrap()
  }

  fn into_napi_handle(self) -> NapiHandle<'a> {
    let handle = self.get_napi_handle();
    let env = handle.env;
    let handle = handle.handle;
    NapiHandle { phantom: PhantomData, env, handle }
  }
}

#[allow(private_bounds)]
pub trait NapiType<'a>: Sized + NapiTypeInternal<'a> {}
