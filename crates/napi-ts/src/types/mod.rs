use crate::errors::*;
use crate::napi;
use std::any::type_name;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;

mod bigint;
mod boolean;
// mod external;
// mod function;
mod macros;
mod null;
mod number;
mod object;
mod properties;
// mod reference;
mod string;
mod symbol;
mod undefined;
mod value;

pub use bigint::*;
pub use boolean::*;
// pub use external::*;
// pub use function::*;
pub use null::*;
pub use number::*;
pub use object::*;
pub use properties::*;
// pub use reference::*;
pub use string::*;
pub use symbol::*;
pub use undefined::*;
pub use value::*;

pub (self) use macros::napi_type;
pub (self) use macros::napi_value;

// ===== CONVERSION ============================================================

pub (crate) trait NapiFrom<'a, T>: Sized {
  fn napi_from(value: T, env: napi::Env) -> Self;
}

pub (crate) trait NapiInto<'a, T>: Sized {
  fn napi_into(self, env: napi::Env) -> T;
}

impl<'a, T, U> NapiInto<'a, U> for T
where
  U: NapiFrom<'a, T>,
{
  fn napi_into(self, env: napi::Env) -> U {
    U::napi_from(self, env)
  }
}

// ===== TYPES =================================================================

#[derive(Debug)]
pub struct NapiRef<'a, T: NapiType + 'a> {
  phantom: PhantomData<&'a mut T>,
  value: T,
}

impl <'a, T: NapiType + 'a> NapiInternal for NapiRef<'a, T> {
  fn from_handle(handle: napi::Handle) -> Self {
    Self { phantom: PhantomData, value: T::from_handle(handle) }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.value.napi_handle()
  }
}

impl <'a, T: NapiType + 'a> Into<NapiOk> for NapiRef<'a, T> {
  fn into(self) -> NapiOk {
    NapiOk { handle: self.napi_handle() }
  }
}

impl <'a, T: NapiType + 'a> Into<NapiErr> for NapiRef<'a, T> {
  fn into(self) -> NapiErr {
    NapiErr {
      message: "JavaScript Error".into(),
      handle: Some(self.napi_handle()),
    }
  }
}

impl <'a, T: NapiType + 'a> Deref for NapiRef<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}

impl <'a, T: NapiType + 'a> From<T> for NapiRef<'a, T> {
  fn from(value: T) -> Self {
    Self { phantom: PhantomData, value }
  }
}

impl <'a, T: NapiType + 'a> NapiRef<'a, T> {
  pub fn as_value(&self) -> NapiRef<'a, NapiValue> {
    let handle = self.napi_handle();
    let value = NapiValue::from_handle(handle);
    NapiRef { phantom: PhantomData, value }
  }
}

impl <'a> NapiRef<'a, NapiValue> {
  pub fn downcast<T: NapiType>(self) -> Result<NapiRef<'a, T>, NapiErr> {
    let value = self.value;
    let t = value.to_string();

    let q: Result<T, <T as TryFrom<NapiValue>>::Error> = T::try_from(value);
    match q {
      Ok(value) => Ok(NapiRef { phantom: PhantomData, value }),
      Err(_) => Err(format!("Unable to downcast {} into {}", t, type_name::<T>()).into()),
    }
  }
}


// ===== TYPES =================================================================

pub (crate) trait NapiInternal: {
  fn from_handle(handle: napi::Handle) -> Self;
  fn napi_handle(&self) -> napi::Handle;
  fn napi_env(&self) -> napi::Env {
    self.napi_handle().env()
  }
}

pub (crate) trait NapiTypeInternal: Into<NapiValue> {
  fn from_handle(handle: napi::Handle) -> Self;
  fn napi_handle(&self) -> napi::Handle;
  fn napi_env(&self) -> napi::Env {
    self.napi_handle().env()
  }
}

#[allow(private_bounds)]
pub trait NapiType: TryFrom<NapiValue> + NapiTypeInternal + Debug + Sized {
  // marker
}
