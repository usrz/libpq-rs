use crate::types::*;
use std::fmt;
use std::cell::OnceCell;


pub struct NapiValue {
  handle: napi::Handle,
  type_of: OnceCell<NapiTypeOf>,
}

impl fmt::Debug for NapiValue {
  fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.type_of.get() {
      Some(type_of) => fm.debug_tuple(&format!("NapiValue<{}>", type_of)),
      None => fm.debug_tuple("NapiValue<Undetermined>"),
    }.field(&self.handle).finish()
  }
}

impl NapiType for NapiValue {}

impl NapiTypeIdInternal for NapiValue {
  fn has_type_of(_: NapiTypeOf) -> bool {
    true
  }

  fn type_of(&self) -> NapiTypeOf {
    self.type_of.get_or_init(|| self.handle.type_of()).clone()
  }
}

impl NapiTypeInternal for NapiValue {
  unsafe fn from_handle(handle: napi::Handle) -> Self {
    NapiValue::from_handle(handle)
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

impl NapiValue {
  pub (crate) fn from_handle(handle: napi::Handle) -> Self {
    Self { handle, type_of: OnceCell::new() }
  }

  pub (crate) fn from_handle_and_type_of(handle: napi::Handle, type_of: NapiTypeOf) -> Self {
    let cell = OnceCell::new();
    cell.set(type_of).unwrap();
    Self { handle, type_of: cell }
  }
}
