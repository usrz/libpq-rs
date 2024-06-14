use crate::napi;
use crate::types::*;

pub struct NapiUndefined {
  handle: napi::Handle,
}

// ===== NAPI TYPE BASICS ======================================================

napi_value!(NapiUndefined, Undefined);

impl NapiTypeInternal for NapiUndefined {
  fn from_handle(handle: napi::Handle) -> Self {
    Self { handle }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== CONVERSION IN =========================================================

impl <'a> NapiFrom<'a, ()> for NapiRef<'a, NapiUndefined> {
  fn napi_from(_: (), env: napi::Env) -> Self {
    NapiUndefined { handle: env.get_undefined() }.into()
  }
}
