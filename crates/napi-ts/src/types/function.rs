use crate::napi;
use crate::types::*;
use crate::errors::NapiResult;

#[derive(Clone,Debug)]
pub struct NapiFunction {
  value: napi::Value,
}

impl NapiValue for NapiFunction {}

impl NapiValueInternal for NapiFunction {
  fn as_napi_value(&self) -> napi::Value {
    self.value
  }

  fn from_napi_value(value: napi::Value) -> Self {
    Self { value }
  }
}

impl Into<NapiResult<NapiValues>> for NapiFunction {
  fn into(self) -> NapiResult<NapiValues> {
    Ok(self.into())
  }
}

impl NapiFunction {
  pub fn new<F>(name: &str, callback: F) -> Self
  where
    F: Fn(NapiValues, Vec<NapiValues>) -> NapiResult<NapiValues> + 'static,
  {
    let value = napi::create_function(name, move |this, args| {
      let this = NapiValues::from_napi_value(this);
      let args: Vec<NapiValues> = args
        .into_iter()
        .map(|value| NapiValues::from_napi_value(value))
        .collect();

      callback(this, args).map(|value| value.as_napi_value())
    });

    Self { value }
  }
}
