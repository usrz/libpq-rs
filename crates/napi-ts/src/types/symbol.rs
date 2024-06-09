
use crate::napi;
use crate::types::*;
use std::marker::PhantomData;
use std::ptr;

pub struct NapiSymbol<'a> {
  phantom: PhantomData<&'a ()>,
  env: napi::Env,
  handle: napi::Handle,
}

impl Debug for NapiSymbol<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiSymbol")
      .field("@", &self.handle)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a> NapiType<'a> for NapiSymbol<'a> {}

impl <'a> NapiTypeInternal<'a> for NapiSymbol<'a> {
  fn from_napi(env: napi::Env, handle: napi::Handle) -> Self {
    Self { phantom: PhantomData, env, handle }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== STRING ================================================================

impl NapiFrom<Option<&str>> for NapiSymbol<'_> {
  fn napi_from(value: Option<&str>, env: napi::Env) -> Self {
    let description = match value {
      Some(description) => napi::create_string_utf8(env, description),
      None => ptr::null_mut(),
    };

    let handle = napi::create_symbol(env, description);
    Self { phantom: PhantomData, env, handle }
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiSymbol<'_> {
  pub fn description(&self) -> Option<String> {
    let key = napi::create_string_utf8(self.env, "description");
    let value = napi::get_property(self.env, self.handle, key);

    match napi::type_of(self.env, value) {
      napi::TypeOf::napi_string => Some(napi::get_value_string_utf8(self.env, value)),
      _ => None,
    }
  }
}
