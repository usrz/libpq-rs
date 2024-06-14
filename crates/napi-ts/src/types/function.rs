use crate::napi;
use crate::types::*;
use crate::context::Context;

#[derive(Clone, Copy)]
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
  pub fn new<F>(env: napi::Env, name: Option<&str>, function: F) -> NapiFunction
  where
    F: Fn(Context, NapiRef<NapiValue>, Vec<NapiRef<NapiValue>>) -> NapiResult + 'static
  {

    let handle = env.create_function(name, move |env, this, args| {
      let env = Context::new(env);
      let this = NapiValue::from_handle(this).as_napi_ref();
      let args = args
        .iter()
        .map(|handle| NapiValue::from_handle(*handle).as_napi_ref())
        .collect();

      let foo = (function)(env, this, args);
      println!("{:?}", foo);
      foo
    });

    Self { handle }
  }
}
