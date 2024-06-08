use crate::napi;
use crate::types::*;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct NapiString {
  value: RefCell<Option<String>>,
  handle: Option<napi::Handle>,
}

impl NapiShape for NapiString {}

impl NapiShapeInternal for NapiString {
  fn into_napi_value(self) -> napi::Handle {
    if let Some(handle) = self.handle {
      return handle
    } else {
      let string = self.value.into_inner().unwrap();
      napi::create_string_utf8(string.as_str())
    }
  }

  fn from_napi_value(handle: napi::Handle) -> Self {
    Self { value: RefCell::new(None), handle: Some(handle) }
  }
}

// ===== &STR CONVERSION =======================================================

impl <S: AsRef<str>> From<S> for NapiString {
  fn from(value: S) -> Self {
    let string = value.as_ref().to_string();
    Self { value: RefCell::new(Some(string)), handle: None }
  }
}

// ===== STRING CONVERSION =====================================================

impl Into<String> for NapiString {
  fn into(self) -> String {
    self.value()
  }
}

// ===== EXTRA METHODS =========================================================

impl NapiString {
  pub fn new<S: AsRef<str>>(string: S) -> Self {
    Self::from(string.as_ref())
  }

  pub fn value(&self) -> String {
    self.value.borrow_mut().get_or_insert_with(|| {
      napi::get_value_string_utf8(self.handle.unwrap())
    }).clone()
  }
}
