use crate::napi;
use std::ptr;

#[derive(Debug)]
pub struct NapiReference {
  value: napi::Handle,
  reference: napi::Reference,
}

impl From<napi::Handle> for NapiReference {
  fn from(value: napi::Handle) -> Self {
    if value.is_null() {
      Self { value, reference: ptr::null_mut() }
    } else {
      Self { value, reference: napi::create_reference(value, 1) }
    }
  }
}

impl Clone for NapiReference {
  fn clone(&self) -> Self {
    match self.value.is_null() {
      false => napi::reference_ref(self.reference),
      true => 0,
    };

    Self { value: self.value, reference: self.reference }
  }
}

impl Drop for NapiReference {
  fn drop(&mut self) {
    let count = match self.value.is_null() {
      false => napi::reference_unref(self.reference),
      true => 0,
    };

    if (count == 0) && (! self.value.is_null()) {
      napi::delete_reference(self.reference)
    }
  }
}

impl NapiReference {
  pub(super) fn value(&self) -> napi::Handle {
    self.value
  }
}
