use crate::napi;

#[derive(Debug)]
pub struct NapiReference {
  value: napi::Value,
  reference: napi::Reference,
  debug: bool
}

impl From<napi::Value> for NapiReference {
  fn from(value: napi::Value) -> Self {
    Self { value, reference: napi::create_reference(value, 1), debug: false }
  }
}

impl Clone for NapiReference {
  fn clone(&self) -> Self {
    let count = napi::reference_ref(self.reference);
    if self.debug {
      println!(">>> CLONED REF {:?} count={}", self.value, count);
    }
    Self { value: self.value, reference: self.reference, debug: self.debug }
  }
}

impl Drop for NapiReference {
  fn drop(&mut self) {
    let count = napi::reference_unref(self.reference);
    if self.debug {
      println!(">>> DROPPED REF {:?} count={}", self.value, count);
    }
    if count == 0 { napi::delete_reference(self.reference) }
  }
}

impl NapiReference {
  pub(super) fn verbose(value: napi::Value) -> Self {
    println!(">>> CREATED REF {:?} count=1", value);
    Self { value, reference: napi::create_reference(value, 1), debug: true }
  }

  pub(super) fn value(&self) -> napi::Value {
    self.value
  }
}
