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

impl NapiTypeWithTypeOf for NapiValue {
  const TYPE_OF: Option<NapiTypeOf> = None;
}

impl NapiTypeInternal for NapiValue {
  unsafe fn from_handle(handle: napi::Handle) -> Result<Self, NapiErr> {
    Ok(NapiValue::from_handle(handle))
  }

  fn from_napi_value(value: &NapiValue) -> Result<Self, NapiErr> {
    Ok(Self { handle: value.handle, type_of: value.type_of.clone() })
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

impl NapiValue {
  pub fn type_of(&self) -> NapiTypeOf {
    self.type_of.get_or_init(|| self.handle.type_of()).clone()
  }

  pub (crate) fn from_handle(handle: napi::Handle) -> Self {
    Self { handle, type_of: OnceCell::new() }
  }

  pub (crate) fn from_handle_and_type_of(handle: napi::Handle, type_of: NapiTypeOf) -> Self {
    let cell = OnceCell::new();
    cell.set(type_of).unwrap();
    Self { handle, type_of: cell }
  }
}
