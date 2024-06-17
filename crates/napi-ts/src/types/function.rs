use crate::contexts::*;
use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiFunction {
  handle: napi::Handle,
}

napi_type!(NapiFunction, Function, {
  unsafe fn from_handle(handle: napi::Handle) -> Result<Self, NapiErr> {
    Ok(Self { handle })
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});

impl <'a> NapiProperties<'a> for NapiRef<'a, NapiFunction> {}

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
    args: &[&NapiRef<'a, NapiValue>],
  ) -> NapiResult<'a, NapiValue> {
    let this = this
      .map(|this| this.napi_handle())
      .unwrap_or_else(|| napi::env().get_null());

    let handles: Vec<napi::Handle> = args
      .into_iter()
      .map(|arg| arg.napi_handle())
      .collect();
    let ehandles: Vec<&napi::Handle> = handles.iter().collect();

    self.handle.call_function(&this, ehandles.as_slice())
      .map(|ok| NapiValue::from_handle(ok).as_napi_ref())
      .map_err(|err| NapiValue::from_handle(err).as_napi_ref().into())
  }

  pub fn with<'a, T: NapiType>(&self, arg: NapiRef<'a, T>) -> NapiArguments<'a> {
    let handle = arg.napi_handle();
    NapiArguments { phantom: PhantomData, function: self.handle, arguments: vec![handle] }
  }
}

pub struct NapiArguments<'a> {
  phantom: PhantomData<&'a ()>,
  function: napi::Handle,
  arguments: Vec<napi::Handle>,
}

impl <'a> NapiArguments<'a> {
  pub fn with<T: NapiType>(&mut self, arg: NapiRef<T>) -> &Self {
    let handle = arg.napi_handle();
    self.arguments.push(handle);
    self
  }

  pub fn call(&self) -> NapiResult<'a, NapiValue> {
    let this = napi::env().get_null();
    let arguments: Vec<&napi::Handle> = self.arguments.iter().collect();

    self.function.call_function(&this, arguments.as_slice())
      .map(|ok| NapiValue::from_handle(ok).as_napi_ref())
      .map_err(|err| NapiValue::from_handle(err).as_napi_ref().into())
  }

  pub fn call_on<T: NapiType>(&self, this: NapiRef<T>) -> NapiResult<NapiValue> {
    let this = this.napi_handle();
    let arguments: Vec<&napi::Handle> = self.arguments.iter().collect();

    self.function.call_function(&this, arguments.as_slice())
      .map(|ok| NapiValue::from_handle(ok).as_napi_ref())
      .map_err(|err| NapiValue::from_handle(err).as_napi_ref().into())
  }

}
