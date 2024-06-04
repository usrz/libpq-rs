use crate::errors::*;
use crate::napi;
use crate::types::*;

#[derive(Clone,Debug)]
pub struct NapiFunction {
  value: napi::Value,
}

impl NapiShape for NapiFunction {}

impl NapiShapeInternal for NapiFunction {
  fn as_napi_value(&self) -> napi::Value {
    self.value
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { value }
  }
}

impl NapiFunction {
  pub fn new<F>(name: &str, callback: F) -> Self
  where
    F: Fn(NapiValue, Vec<NapiValue>) -> NapiResult<NapiReturn> + 'static,
  {
    let value = napi::create_function(name, move |this, args| {
      let this = NapiValue::from_napi_value(this);
      let args: Vec<NapiValue> = args
        .into_iter()
        .map(|value| NapiValue::from_napi_value(value))
        .collect();

      callback(this, args).map(|value| value.as_napi_value())
    });

    Self { value }
  }

  pub fn call(&self, args: &[&impl NapiShape]) -> NapiResult<NapiReturn> {
    self.call_with(&NapiNull::new(), args)
  }

  pub fn call_with(&self, this: &impl NapiShape, args: &[&impl NapiShape]) -> NapiResult<NapiReturn> {
    let args = args
      .into_iter()
      .map(|value| value.as_napi_value())
      .collect();

    napi::call_function(this.as_napi_value(), self.value, args)
      .map(|value| NapiReturn::from_napi_value(value))
  }
}
