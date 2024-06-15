use crate::TypeOf;
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
use std::any::type_name;

// ===== TYPES =================================================================

pub (crate) trait NapiRefInternal {
  fn napi_handle(&self) -> napi::Handle;
  fn napi_env(&self) -> napi::Env {
    self.napi_handle().env()
  }
}


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
    T::from_napi_value(&self.value.as_napi_value())
      .map(|value| value.as_napi_ref())
  }
}


// ===== PRIVATE TRAITS ========================================================

pub (crate) trait NapiTypeIdInternal {
  fn has_type_of(type_of: TypeOf) -> bool;

  fn type_of(&self) -> TypeOf;
}

pub (crate) trait NapiTypeInternal: NapiTypeIdInternal + fmt::Debug + Sized {
  fn napi_handle(&self) -> napi::Handle;

  unsafe fn from_handle(handle: napi::Handle) -> Self;

  fn from_napi_value(value: &NapiValue) -> Result<Self, NapiErr> {
    if Self::has_type_of(value.type_of()) {
      return Ok(unsafe { Self::from_handle(value.napi_handle()) })
    }

    Err(format!("Unable to downcast {:?} into {}", value.type_of(), type_name::<Self>()).into())
  }

  fn as_napi_value(&self) -> NapiValue {
    NapiValue::from_handle_and_type_of(self.napi_handle(), self.type_of())
  }

  fn as_napi_ref<'a>(self) -> NapiRef<'a, Self> {
    NapiRef { phantom: PhantomData, value: self }
  }
}

// ===== PUBLIC TRAITS =========================================================

#[allow(private_bounds)]
pub trait NapiType: NapiTypeInternal {
  // Marker type
}
