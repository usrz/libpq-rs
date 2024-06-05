use crate::errors::*;
use crate::napi;
use crate::types::*;

#[derive(Debug)]
pub struct NapiFunction {
  value: napi::Value,
  reference: napi::Reference,
}

impl NapiShape for NapiFunction {}

impl Clone for NapiFunction {
  fn clone(&self) -> Self {
    napi::reference_ref(self.reference);
    Self { value: self.value, reference: self.reference }
  }
}

impl Drop for NapiFunction {
  fn drop(&mut self) {
    napi::reference_unref(self.reference);
  }
}

impl NapiShapeInternal for NapiFunction {
  fn as_napi_value(self) -> napi::Value {
    self.value
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { value, reference: napi::create_reference(value, 1) }
  }
}

impl NapiFunction {
  pub fn new<F>(name: &str, callback: F) -> Self
  where
    F: Fn(NapiValue, Vec<NapiValue>) -> NapiResult<NapiReturn> + Send + 'static,
  {
    let value = napi::create_function(name, move |this, args| {
      let this = NapiValue::from_napi_value(this);
      let args: Vec<NapiValue> = args
        .into_iter()
        .map(|value| NapiValue::from_napi_value(value))
        .collect();

      callback(this, args).map(|value| value.as_napi_value())
    });

    Self::from_napi_value(value)
  }

  pub fn call(&self, args: &[impl NapiShape]) -> NapiResult<NapiReturn> {
    self.call_with(&NapiNull::new(), args)
  }

  pub fn call_with(&self, this: &impl NapiShape, args: &[impl NapiShape]) -> NapiResult<NapiReturn> {
    let args = args
      .into_iter()
      .map(|value| value.clone().as_napi_value())
      .collect();

    napi::call_function(this.clone().as_napi_value(), self.value, args)
      .map(|value| NapiReturn::from_napi_value(value))
  }
}
