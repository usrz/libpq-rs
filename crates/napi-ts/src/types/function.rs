use crate::contexts::*;
use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiFunction {
  handle: napi::Handle,
}

napi_type!(NapiFunction, Function, {
  unsafe fn from_handle(handle: napi::Handle) -> Self {
    Self { handle }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});

// ===== FUNCTION ==============================================================

impl NapiFunction {
  pub fn new<F, T>(env: napi::Env, name: Option<&str>, function: F) -> NapiFunction
  where
    F: Fn(FunctionContext) -> NapiResult<T> + 'static,
    T: NapiType,
  {

    let handle = env.create_function(name, move |env, this, args| {
      let context = FunctionContext::new(env, this, args);
      let result = (function)(context);
      println!("{:?}", result); // TODO cleanup
      result.map(|value| value.napi_handle())
    });

    Self { handle }
  }

  pub fn call<'a>(
    &self,
    this: Option<NapiRef<'a, NapiValue>>,
    args: Vec<NapiRef<'a, NapiValue>>,
  ) -> NapiResult<'a, NapiValue> {
    let this = this
      .map(|this| this.napi_handle())
      .unwrap_or_else(|| self.handle.env().get_null());

    let args = args
      .into_iter()
      .map(|arg| arg.napi_handle())
      .collect();

    println!("ABOUT TO CALL NOW!!!");
    self.napi_handle().call_function(&this, args)
      .map(|ok| NapiValue::from_handle(ok).as_napi_ref())
      .map_err(|err| NapiValue::from_handle(err).as_napi_ref().into())
  }
}
