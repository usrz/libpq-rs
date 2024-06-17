use crate::NapiTypeOf;
use crate::errors::*;
use crate::napi;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;

mod array;
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

pub use array::*;
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
}


#[allow(private_bounds)]
pub struct NapiRef<'a, T: NapiTypeInternal<'a>> {
  phantom: PhantomData<&'a mut T>,
  value: T,
}

impl <'a, T: NapiType<'a>> fmt::Debug for NapiRef<'a, T> {
  fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
    fm.debug_tuple("NapiRef")
      .field(&self.value)
      .finish()
  }
}

impl <'a, T: NapiType<'a>> NapiRefInternal for NapiRef<'a, T> {
  fn napi_handle(&self) -> napi::Handle {
    self.value.napi_handle()
  }
}

impl <'a, T: NapiType<'a>> Into<NapiErr> for NapiRef<'a, T> {
  fn into(self) -> NapiErr {
    NapiErr::from_handle(self.napi_handle())
  }
}

impl <'a, T: NapiType<'a>> Deref for NapiRef<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}

impl <'a, T: NapiType<'a>> NapiRef<'a, T> {
  pub fn as_value(&self) -> NapiRef<'a, NapiValue<'a>> {
    let handle = self.napi_handle();
    let value = NapiValue::from_handle(handle);
    NapiRef { phantom: PhantomData, value }
  }

  pub fn downcast<D: NapiType<'a>>(&self) -> Result<NapiRef<'a, D>, NapiErr> {
    D::from_napi_value(&self.value.as_napi_value())
      .map(|value| value.as_napi_ref())
  }
}

// ===== PRIVATE TRAITS ========================================================

pub (crate) trait NapiTypeWithTypeOf {
  const TYPE_OF: Option<NapiTypeOf>;
}

pub (crate) trait NapiTypeInternal<'a>: NapiTypeWithTypeOf + fmt::Debug + Sized {
  fn napi_handle(&self) -> napi::Handle;

  unsafe fn from_handle(handle: napi::Handle) -> Result<Self, NapiErr>;

  fn from_napi_value(value: &NapiValue) -> Result<Self, NapiErr> {
    if let Some(type_of) = Self::TYPE_OF {
      if type_of != value.type_of() {
        return Err(format!("Invalid handle type {:?} for {}", value.type_of(), type_name::<Self>()).into())
      }
    }

    return Ok(unsafe { Self::from_handle(value.napi_handle())? })
  }

  fn as_napi_value(&self) -> NapiValue {
    let handle = self.napi_handle();
    match Self::TYPE_OF {
      Some(type_of) => NapiValue::from_handle_and_type_of(handle, type_of),
      None => NapiValue::from_handle(handle),
    }
  }

  fn as_napi_ref(self) -> NapiRef<'a, Self> {
    NapiRef { phantom: PhantomData, value: self }
  }
}

// ===== PUBLIC TRAITS =========================================================

#[allow(private_bounds)]
pub trait NapiType<'a>: NapiTypeInternal<'a> {
  // Marker type
}
