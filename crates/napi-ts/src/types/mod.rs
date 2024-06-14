use crate::context::*;
use crate::errors::*;
use crate::napi;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;

mod bigint;
mod boolean;
mod external;
mod function;
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
pub use external::*;
pub use function::*;
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

// ===== TYPES =================================================================

#[allow(private_bounds)]
pub struct NapiRef<'a, T: NapiTypeInternal + 'a> {
  phantom: PhantomData<&'a mut T>,
  value: T,
}

impl <T: NapiType> fmt::Debug for NapiRef<'_, T> {
  fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
    fm.debug_tuple("NapiRef")
      .field(&self.value)
      .finish()
  }
}

impl <'a, T: NapiType + 'a> NapiRefInternal for NapiRef<'a, T> {
  #[inline]
  fn from_handle(handle: napi::Handle) -> Self {
    Self { phantom: PhantomData, value: T::from_handle(handle) }
  }

  #[inline]
  fn napi_handle(&self) -> napi::Handle {
    self.value.napi_handle()
  }
}

impl <'a, T: NapiType + 'a> Into<NapiErr> for NapiRef<'a, T> {
  #[inline]
  fn into(self) -> NapiErr {
    NapiErr::from_handle(self.napi_handle())
  }
}

impl <'a, T: NapiType + 'a> Deref for NapiRef<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}

impl <'a, T: NapiType + 'a> NapiRef<'a, T> {
  #[inline]
  pub fn as_value(&self) -> NapiRef<'a, NapiValue> {
    let handle = self.napi_handle();
    let value = NapiValue::from_handle(handle);
    NapiRef { phantom: PhantomData, value }
  }
}

impl <'a> NapiRef<'a, NapiValue> {
  #[inline]
  pub fn downcast<T: NapiType>(&self) -> NapiResult<'a, T> {
    T::try_from_napi_value(&self.value).map(|value| value.as_napi_ref())
  }
}


// ===== TYPES =================================================================

pub (crate) trait NapiRefInternal {
  fn from_handle(handle: napi::Handle) -> Self;
  fn napi_handle(&self) -> napi::Handle;
  fn napi_env(&self) -> napi::Env {
    self.napi_handle().env()
  }
}

pub (crate) trait NapiTypeInternal: Into<NapiValue> + fmt::Debug + Sized {
  fn from_handle(handle: napi::Handle) -> Self;
  fn napi_handle(&self) -> napi::Handle;
  fn as_napi_ref<'a>(self) -> NapiRef<'a, Self> {
    NapiRef { phantom: PhantomData, value: self }
  }
}

#[allow(private_bounds)]
pub trait NapiType: NapiTypeInternal {
  fn into_napi_value(self) -> NapiValue;
  fn try_from_napi_value(value: &NapiValue) -> Result<Self, NapiErr>;
}
