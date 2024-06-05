use crate::napi;

#[derive(Debug)]
pub struct NapiReference {
  value: napi::Value,
  reference: napi::Reference,
}

impl From<napi::Value> for NapiReference {
  fn from(value: napi::Value) -> Self {
    println!(">>> CREATED REF {:?} count=1", value);
    Self { value, reference: napi::create_reference(value, 1) }
  }
}

impl Clone for NapiReference {
  fn clone(&self) -> Self {
    let count = napi::reference_ref(self.reference);
    println!(">>> CLONED REF {:?} count={}", self.value, count);
    Self { value: self.value, reference: self.reference }
  }
}

impl Drop for NapiReference {
  fn drop(&mut self) {
    let count = napi::reference_unref(self.reference);
    println!(">>> DROPPED REF {:?} count={}", self.value, count);
    if count == 0 { napi::delete_reference(self.reference) }
  }
}

impl NapiReference {
  pub(crate) fn value(&self) -> napi::Value {
    self.value
  }
}
