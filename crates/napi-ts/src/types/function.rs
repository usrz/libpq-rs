use crate::errors::*;
use crate::napi;
use crate::types::*;

#[derive(Clone)]
pub struct NapiFunction {
  reference: NapiReference,
}

impl Debug for NapiFunction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NapiFunction")
      .field("@", &self.reference.handle())
      .finish()
  }
}

impl NapiShape for NapiFunction {}

impl NapiShapeInternal for NapiFunction {
  fn into_napi_value(self) -> napi::Handle {
    self.reference.handle()
  }

  fn from_napi_value(handle: napi::Handle) -> Self {
    napi::expect_type_of(handle, napi::Type::napi_function);
    Self { reference: handle.into() }
  }
}

impl NapiFunction {
  pub fn new<F>(callback: F) -> Self
  where
    F: Fn(NapiValue, Vec<NapiValue>) -> NapiResult<NapiReturn> + Send + 'static,
  {
    Self::named("", callback)
  }

  pub fn named<F>(name: &str, callback: F) -> Self
  where
    F: Fn(NapiValue, Vec<NapiValue>) -> NapiResult<NapiReturn> + Send + 'static,
  {
    let handle = napi::create_function(name, move |this, args| {
      let this = NapiValue::from(this);
      let args: Vec<NapiValue> = args
        .into_iter()
        .map(|value| NapiValue::from(value))
        .collect();

      callback(this, args).map(|ret| ret.into())
    });

    Self::from_napi_value(handle)
  }

  pub fn call(&self, args: &[impl NapiShape]) -> NapiResult<NapiReturn> {
    self.call_with(&NapiNull::new(), args)
  }

  pub fn call_with(&self, this: &impl NapiShape, args: &[impl NapiShape]) -> NapiResult<NapiReturn> {
    let args = args
      .into_iter()
      .map(|value| value.clone().into_napi_value())
      .collect();

    napi::call_function(this.clone().into_napi_value(), self.reference.handle(), args)
      .map(|value| value.into())
  }
}
