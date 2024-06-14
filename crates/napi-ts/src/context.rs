use crate::errors::*;
use crate::napi;
use crate::types::*;
use std::fmt;
use std::marker::PhantomData;

pub (crate) trait NapiContextInternal<'a> {
  fn napi_env(&self) -> napi::Env;
}

#[allow(private_bounds)]
pub trait NapiContext<'a>: NapiContextInternal<'a> {
  fn bigint<V: Into<i128>>(&self, value: V) -> NapiRef<'a, NapiBigint> {
    NapiBigint::new(self.napi_env(), value.into()).as_napi_ref()
  }

  fn boolean<V: Into<bool>>(&self, value: V) -> NapiRef<'a, NapiBoolean> {
    NapiBoolean::new(self.napi_env(), value.into()).as_napi_ref()
  }

  fn external<T: 'static>(&self, data: T) -> NapiRef<'a, NapiExternal<T>> {
    NapiExternal::new(self.napi_env(), data).as_napi_ref()
  }

  fn function<F, T>(&self, function: F) -> NapiRef<'a, NapiFunction>
  where
    F: Fn(FunctionContext) -> NapiResult<T> + 'static,
    T: NapiType
  {
    NapiFunction::new(self.napi_env(), None, function).as_napi_ref()
  }

  fn null(&self) -> NapiRef<'a, NapiNull> {
    NapiNull::new(self.napi_env()).as_napi_ref()
  }

  fn number<V: Into<f64>>(&self, value: V) -> NapiRef<'a, NapiNumber> {
    NapiNumber::new(self.napi_env(), value.into()).as_napi_ref()
  }

  fn object(&self) -> NapiRef<'a, NapiObject> {
    NapiObject::new(self.napi_env()).as_napi_ref()
  }

  fn string<T: AsRef<str>>(&self, value: T) -> NapiRef<'a, NapiString> {
    NapiString::new(self.napi_env(), value.as_ref()).as_napi_ref()
  }

  fn symbol<T: AsRef<str>>(&self, description: Option<T>) -> NapiRef<'a, NapiSymbol> {
    match description {
      Some(str) => NapiSymbol::new(self.napi_env(), Some(str.as_ref())),
      None => NapiSymbol::new(self.napi_env(), None),
    }.as_napi_ref()
  }

  fn symbol_for<T: AsRef<str>>(&self, description: T) -> NapiRef<'a, NapiSymbol> {
    NapiSymbol::new_for(self.napi_env(), description.as_ref()).as_napi_ref()
  }

  fn undefined(&self) -> NapiRef<'a, NapiUndefined> {
    NapiUndefined::new(self.napi_env()).as_napi_ref()
  }
}

// ========================================================================== //
// INIT CONTEXT                                                               //
// ========================================================================== //

pub struct Context<'a> {
  phantom: PhantomData<&'a mut ()>,
  env: napi::Env,
  exports: napi::Handle,
}

impl fmt::Debug for Context<'_> {
  fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
    fm.debug_tuple("InitContext")
      .field(&self.env)
      .finish()
  }
}

impl <'a> NapiContext<'a> for Context<'a> {}
impl <'a> NapiContextInternal<'a> for Context<'a> {
  #[inline]
  fn napi_env(&self) -> napi::Env {
    self.env
  }
}

impl <'a> Context<'a> {
  pub (crate) fn new(env: napi::Env, exports: napi::Handle) -> Self {
    Self { phantom: PhantomData, env, exports }
  }

  pub fn exports(&self) -> NapiRef<'a, NapiObject> {
    NapiObject::from_handle(self.exports).as_napi_ref()
  }
}

// ========================================================================== //
// FUNCTION CONTEXT                                                           //
// ========================================================================== //

pub struct FunctionContext<'a> {
  phantom: PhantomData<&'a mut ()>,
  env: napi::Env,
  this: napi::Handle,
  args: Vec<napi::Handle>,
}

impl fmt::Debug for FunctionContext<'_> {
  fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
    fm.debug_tuple("InitContext")
      .field(&self.env)
      .finish()
  }
}

impl <'a> NapiContext<'a> for FunctionContext<'a> {}
impl <'a> NapiContextInternal<'a> for FunctionContext<'a> {
  #[inline]
  fn napi_env(&self) -> napi::Env {
    self.env
  }
}

impl <'a> FunctionContext<'a> {
  pub (crate) fn new(env: napi::Env, this: napi::Handle, args: Vec<napi::Handle>) -> Self {
    Self { phantom: PhantomData, env, this, args: args.to_vec() }
  }

  pub fn this(&self) -> NapiRef<'a, NapiValue> {
    NapiValue::from_handle(self.this).as_napi_ref()
  }

  pub fn args(&self) -> Vec<NapiRef<'a, NapiValue>> {
    self.args
      .iter()
      .map(|handle| NapiValue::from_handle(*handle).as_napi_ref())
      .collect()
  }
}
