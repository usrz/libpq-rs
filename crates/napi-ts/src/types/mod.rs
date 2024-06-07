use crate::napi;
use std::fmt::Debug;

mod bigint;
mod boolean;
mod external;
mod function;
mod null;
mod number;
mod object;
mod reference;
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
pub use string::*;
pub use symbol::*;
pub use undefined::*;
pub use value::*;

pub(self) use reference::*;
use crate::NapiResult;

pub(self) trait NapiShapeInternal: Clone + Debug {
  fn into_napi_value(self) -> napi::Value;
  fn from_napi_value(value: napi::Value) -> Self;

  unsafe fn downcast_external<T: NapiShape + 'static>(&self, _: napi::Value) -> NapiResult<Self> {
    panic!("Attempted to invoke \"downcast_external\" on {}", std::any::type_name::<Self>());
  }
}

#[allow(private_bounds)]
pub trait NapiShape: NapiShapeInternal {
  // Marker
}
