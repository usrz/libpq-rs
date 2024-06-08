use crate::napi;
use std::ptr;

#[derive(Debug)]
pub struct NapiReference {
  handle: napi::Handle,
  reference: napi::Reference,
}

impl From<napi::Handle> for NapiReference {
  fn from(handle: napi::Handle) -> Self {
    if handle.is_null() {
      Self { handle, reference: ptr::null_mut() }
    } else {
      Self { handle, reference: napi::create_reference(handle, 1) }
    }
  }
}

impl Clone for NapiReference {
  fn clone(&self) -> Self {
    match self.handle.is_null() {
      false => napi::reference_ref(self.reference),
      true => 0,
    };

    Self { handle: self.handle, reference: self.reference }
  }
}

impl Drop for NapiReference {
  fn drop(&mut self) {
    let count = match self.handle.is_null() {
      false => napi::reference_unref(self.reference),
      true => 0,
    };

    if (count == 0) && (! self.handle.is_null()) {
      napi::delete_reference(self.reference)
    }
  }
}

impl NapiReference {
  pub(super) fn handle(&self) -> napi::Handle {
    self.handle
  }
}
