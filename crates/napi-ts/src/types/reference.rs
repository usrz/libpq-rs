use crate::napi;
use std::fmt::Debug;

pub struct NapiReference {
  value: Option<(napi::Handle, napi::Reference)>
}

impl Debug for NapiReference {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self.value {
      None => f
        .debug_struct("NapiReference")
        .field("@", &"null")
        .finish_non_exhaustive(),
      Some(value) => f
        .debug_struct("NapiReference")
        .field("@", &value.0)
        .field("ref", &value.1)
        .finish(),
    }
  }
}

impl From<napi::Handle> for NapiReference {
  fn from(handle: napi::Handle) -> Self {
    if handle.is_null() {
      Self { value: None }
    } else {
      Self { value: Some((handle, napi::create_reference(handle, 1))) }
    }
  }
}

impl Clone for NapiReference {
  fn clone(&self) -> Self {
    match self.value.clone() {
      None => panic!("Attempting to clone null (pseudo) NapiReference"),
      Some((handle, reference)) => {
        napi::reference_ref(reference);
        Self { value: Some((handle, reference)) }
      }
    }
  }
}

impl Drop for NapiReference {
  fn drop(&mut self) {
    if let Some((_, reference)) = self.value {
      let count = napi::reference_unref(reference);
      if count == 0 {
        napi::delete_reference(reference)
      }
    }
  }
}

impl NapiReference {
  pub(super) fn handle(&self) -> napi::Handle {
    match self.value {
      None => panic!("Attempting to get handle from (pseudo) NapiReference"),
      Some(value) => value.0,
    }
  }

  pub(super) fn expect_uninit(&self) {
    if self.value.is_some() {
      panic!("NapiExternal already initialized")
    }
  }
}
