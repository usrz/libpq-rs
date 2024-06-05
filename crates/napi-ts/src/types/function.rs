use crate::errors::*;
use crate::napi;
use crate::types::*;

#[derive(Clone, Debug)]
pub struct NapiFunction {
  value: NapiReference,
}

impl NapiShape for NapiFunction {}

impl NapiShapeInternal for NapiFunction {
  fn into_napi_value(self) -> napi::Value {
    self.value.value()
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { value: value.into() }
  }
}

impl NapiFunction {
  pub fn new<F>(name: &str, callback: F) -> Self
  where
    F: Fn(NapiValue, Vec<NapiValue>) -> NapiResult<NapiReturn> + Send + 'static,
  {
    let value = napi::create_function(name, move |this, args| {
      let this = NapiValue::from(this);
      let args: Vec<NapiValue> = args
        .into_iter()
        .map(|value| NapiValue::from(value))
        .collect();

      callback(this, args).map(|ret| ret.into())
    });

    Self::from_napi_value(value)
  }

  pub fn call(&self, args: &[impl NapiShape]) -> NapiResult<NapiReturn> {
    self.call_with(&NapiNull::new(), args)
  }

  pub fn call_with(&self, this: &impl NapiShape, args: &[impl NapiShape]) -> NapiResult<NapiReturn> {
    let args = args
      .into_iter()
      .map(|value| value.clone().into_napi_value())
      .collect();

    napi::call_function(this.clone().into_napi_value(), self.value.value(), args)
      .map(|value| value.into())
  }
}
