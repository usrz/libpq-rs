use crate::napi;
use crate::types::*;
use crate::context::Context;

pub struct NapiFunction {
  handle: napi::Handle,
}

// ===== NAPI TYPE BASICS ======================================================

napi_type!(NapiFunction, Function);

impl NapiTypeInternal for NapiFunction {
  fn from_handle(handle: napi::Handle) -> Self {
    Self { handle }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== FUNCTION ==============================================================

impl NapiFunction {
  pub fn new<'a, F, T>(env: napi::Env, name: Option<&str>, function: F) -> NapiFunction
  where
    F: Fn(Context<'a>,
          NapiRef<'a, NapiValue>,
          Vec<NapiRef<'a, NapiValue>>
       ) -> NapiResult<'a, T> + 'static,
    T: NapiType + 'a,
  {

    let handle = env.create_function(name, move |env, this, args| {
      let env = Context::new(env);
      let this = NapiValue::from_handle(this).as_napi_ref();
      let args = args
        .iter()
        .map(|handle| NapiValue::from_handle(*handle).as_napi_ref())
        .collect();

      let result = (function)(env, this, args);
      println!("{:?}", result); // TODO cleanup
      result.map(|value| value.napi_handle())
    });

    Self { handle }
  }
}
