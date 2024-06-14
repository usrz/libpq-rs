use crate::napi;
use crate::types::*;

pub struct NapiBoolean {
  handle: napi::Handle,
  value: bool,
}

// ===== NAPI TYPE BASICS ======================================================

napi_value!(NapiBoolean, Boolean);

impl NapiTypeInternal for NapiBoolean {
  #[inline]
  fn from_handle(handle: napi::Handle) -> Self {
    Self { handle, value: handle.get_value_bool() }
  }

  #[inline]
  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== CONVERSION OUT ========================================================

impl NapiBoolean {
  pub fn value(&self) -> bool {
    self.value
  }
}

// ===== CONVERSION IN =========================================================

impl <'a> NapiFrom<'a, bool> for NapiRef<'a, NapiBoolean> {
  fn napi_from(value: bool, env: napi::Env) -> Self {
    NapiBoolean { handle: env.get_boolean(value), value }.into()
  }
}
