use crate::contexts::*;
use crate::types::*;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiFunction<'a> {
  phantom: PhantomData<&'a ()>,
  handle: napi::Handle,
}

napi_type!(NapiFunction, Function, {
  unsafe fn from_handle(handle: napi::Handle) -> Result<Self, NapiErr> {
    Ok(Self { phantom: PhantomData, handle })
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});

impl <'a> NapiProperties<'a> for NapiFunction<'a> {}

// ===== FUNCTION ==============================================================

impl <'a> NapiFunction<'a> {
  pub fn new<'b, F, R>(env: napi::Env, name: Option<&str>, function: F) -> NapiFunction<'a>
  where
    'a: 'b,
    F: (Fn(FunctionContext<'b>) -> NapiResult2<'b, R>) + 'static,
    R: NapiType<'b> + 'b,
  {
    let handle = env.create_function(name, move |env, this, args| {
      let context = FunctionContext::new(env, this, args);
      let result = (function)(context);
      println!("{:?}", result); // TODO cleanup
      result
        .map_err(|err| err.into_handle())
        .map(|ok| ok.napi_handle())
    });

    Self { phantom: PhantomData, handle }
  }

  pub fn with<T: NapiType<'a>>(&'a self, arg: NapiRef<'a, T>) -> NapiArguments<'a> {
    let handle = arg.napi_handle();
    NapiArguments { phantom: PhantomData, function: self.handle, arguments: vec![handle] }
  }
}

// ===== ARGUMENTS =============================================================

pub struct NapiArguments<'a> {
  phantom: PhantomData<&'a ()>,
  function: napi::Handle,
  arguments: Vec<napi::Handle>,
}

impl <'a> NapiArguments<'a> {
  pub fn with<T: NapiType<'a>>(mut self, arg: &NapiRef<'a, T>) -> Self {
    let handle = arg.napi_handle();
    self.arguments.push(handle);
    self
  }

  pub fn call(self) -> NapiResult2<'a, NapiValue<'a>> {
    let this = napi::env().get_null();
    let arguments: Vec<&napi::Handle> = self.arguments.iter().collect();

    self.function.call_function(&this, arguments.as_slice())
      .map(|handle| NapiValue::from_handle(handle).as_napi_ref())
      .map_err(|handle| NapiErr::from_handle(handle))
  }

  pub fn call_on<T: NapiType<'a>>(self, this: NapiRef<'a, T>) -> NapiResult2<'a, NapiValue<'a>>{
    let this = this.napi_handle();
    let arguments: Vec<&napi::Handle> = self.arguments.iter().collect();

    self.function.call_function(&this, arguments.as_slice())
      .map(|handle| NapiValue::from_handle(handle).as_napi_ref())
      .map_err(|handle| NapiErr::from_handle(handle))
  }
}
