use crate::napi;
use crate::types::*;
use crate::errors::NapiError;
use crate::errors::NapiResult;

#[derive(Clone,Debug)]
pub struct NapiFunction {
  pub(super) value: napi::Value,
}

impl NapiValue for NapiFunction {
  unsafe fn as_napi_value(&self) -> napi::Value {
    self.value
  }
}

impl Into<NapiResult<NapiValues>> for NapiFunction {
  fn into(self) -> NapiResult<NapiValues> {
    Ok(self.into())
  }
}

impl TryFrom<napi::Value> for NapiFunction {
  type Error = NapiError;

  fn try_from(value: napi::Value) -> NapiResult<Self> {
    Ok(Self { value: expect_type(value, napi::ValueType::napi_function)? })
  }
}

impl NapiFunction {
  pub fn new<F>(name: &str, callback: F) -> Self
  where
    F: Fn(NapiValues, Vec<NapiValues>) -> NapiResult<NapiValues> + 'static,
  {
    let value = napi::create_function(name, move |this, args| {
      let this = NapiValues::from(this);
      let args: Vec<NapiValues> = args
        .into_iter()
        .map(|value| NapiValues::from(value))
        .collect();

      callback(this, args).map(|value| unsafe { value.as_napi_value() })
    });

    Self { value }
  }
}
