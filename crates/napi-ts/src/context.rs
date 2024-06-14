use crate::napi;
use crate::types::*;
use std::fmt::Debug;
use std::marker::PhantomData;
use crate::NapiResult;

pub struct Context<'a> {
  phantom: PhantomData<&'a mut ()>,
  env: napi::Env,
}

impl Debug for Context<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Context")
      .field("@", &self.env).finish()
  }
}

#[allow(private_bounds)]
impl <'a> Context<'a> {
  pub fn bigint<V: Into<i128>>(&self, value: V) -> NapiRef<'a, NapiBigint> {
    NapiBigint::new(self.env, value.into()).as_napi_ref()
  }

  pub fn boolean<V: Into<bool>>(&self, value: V) -> NapiRef<'a, NapiBoolean> {
    NapiBoolean::new(self.env, value.into()).as_napi_ref()
  }

  // pub fn external<T: 'static>(&self, value: T) -> NapiExternal<'a, T> {
  //   value.napi_into(self.env)
  // }

  pub fn function<F>(&self, function: F) -> NapiRef<'a, NapiFunction> where
    F: Fn(Context, NapiRef<NapiValue>, Vec<NapiRef<NapiValue>>) -> NapiResult + 'static
  {
    NapiFunction::new(self.env, None, function).as_napi_ref()
  }

  pub fn null(&self) -> NapiRef<'a, NapiNull> {
    NapiNull::new(self.env).as_napi_ref()
  }

  pub fn number<V: Into<f64>>(&self, value: V) -> NapiRef<'a, NapiNumber> {
    NapiNumber::new(self.env, value.into()).as_napi_ref()
  }

  pub fn object(&self) -> NapiRef<'a, NapiObject> {
    NapiObject::new(self.env).as_napi_ref()
  }

  pub fn string<T: AsRef<str>>(&self, value: T) -> NapiRef<'a, NapiString> {
    NapiString::new(self.env, value.as_ref()).as_napi_ref()
  }

  pub fn symbol<T: AsRef<str>>(&self, description: Option<T>) -> NapiRef<'a, NapiSymbol> {
    match description {
      Some(str) => NapiSymbol::new(self.env, Some(str.as_ref())),
      None => NapiSymbol::new(self.env, None),
    }.as_napi_ref()
  }

  pub fn symbol_for<T: AsRef<str>>(&self, description: T) -> NapiRef<'a, NapiSymbol> {
    NapiSymbol::new_for(self.env, description.as_ref()).as_napi_ref()
  }

  pub fn undefined(&self) -> NapiRef<'a, NapiUndefined> {
    NapiUndefined::new(self.env).as_napi_ref()
  }
}

impl <'a> Context<'a> {
  pub (crate) fn new(env: napi::Env) -> Self {
    Self { phantom: PhantomData, env }
  }
}
