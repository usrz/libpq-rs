use crate::napi;
use crate::types::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct NapiObject<'a> {
  phantom: PhantomData<&'a ()>,
  env: napi::Env,
  handle: napi::Handle,
}

// impl Debug for NapiObject<'_> {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     f.debug_struct("NapiObject")
//       .field("@", &self.handle)
//       .finish()
//   }
// }

// ===== NAPI::HANDLE CONVERSION ===============================================

impl NapiType for NapiObject<'_> {}

impl NapiTypeInternal for NapiObject<'_> {
  fn handle(&self) -> napi::Handle {
    self.handle
  }
}

impl NapiFrom<napi::Handle> for NapiObject<'_> {
  fn napi_from(handle: napi::Handle, env: napi::Env) -> Self {
    Self { phantom: PhantomData, env, handle }
  }
}

impl NapiInto<napi::Handle> for NapiObject<'_> {
  fn napi_into(self, _env: napi::Env) -> napi::Handle {
    self.handle
  }
}

// ===== OBJECT ================================================================

impl NapiFrom<()> for NapiObject<'_> {
  fn napi_from(_: (), env: napi::Env) -> Self {
    let handle = napi::create_object(env);
    Self { phantom: PhantomData, env, handle }
  }
}

// ===== PROPERTIES ============================================================

impl NapiObject<'_> {
  // fn get_property(&self, key: &str) -> Option<NapiValue> {
  //   let key = napi::create_string_utf8(key);
  //   let this = self.clone().into_napi_value();
  //   let result = napi::get_property(this, key);
  //   let value = NapiValue::from(result);

  //   match value {
  //     NapiValue::Undefined(_) => None,
  //     value => Some(value),
  //   }
  // }

  #[allow(private_bounds)]
  pub fn set_property<T: NapiTypeInternal>(&self, key: &str, value: &T) -> &Self {
    let key = napi::create_string_utf8(self.env, key);
    let value: napi::Handle = value.handle();
    napi::set_property(self.env, self.handle, key, value);
    self
  }

}
