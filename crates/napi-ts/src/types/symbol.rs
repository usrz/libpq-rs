
use crate::napi;
use crate::types::*;
use std::marker::PhantomData;
use std::ptr;

#[derive(Debug)]
pub struct NapiSymbol<'a> {
  phantom: PhantomData<&'a ()>,
  env: napi::Env,
  handle: napi::Handle,
}

// impl Debug for NapiSymbol<'_> {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     f.debug_struct("NapiSymbol")
//       .field("@", &self.handle)
//       .finish()
//   }
// }

// ===== NAPI::HANDLE CONVERSION ===============================================

impl NapiType for NapiSymbol<'_> {}

impl NapiFrom<napi::Handle> for NapiSymbol<'_> {
  fn napi_from(handle: napi::Handle, env: napi::Env) -> Self {
    Self { phantom: PhantomData, env, handle }
  }
}

impl NapiInto<napi::Handle> for NapiSymbol<'_> {
  fn napi_into(self, _env: napi::Env) -> napi::Handle {
    self.handle
  }
}

// ===== STRING ================================================================

impl NapiFrom<Option<&str>> for NapiSymbol<'_> {
  fn napi_from(value: Option<&str>, env: napi::Env) -> Self {
    let description = match value {
      Some(description) => napi::create_string_utf8(description),
      None => ptr::null_mut(),
    };

    let handle = napi::create_symbol(description);
    Self { phantom: PhantomData, env, handle }
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiSymbol<'_> {
  pub fn description(&self) -> Option<String> {
    let key = napi::create_string_utf8("description");
    let value = napi::get_property(self.handle, key);

    match napi::type_of(value) {
      napi::TypeOf::napi_string => Some(napi::get_value_string_utf8(value)),
      _ => None,
    }
  }
}
