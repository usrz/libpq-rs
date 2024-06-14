use crate::errors::*;
use crate::napi;
use crate::types::*;
use std::fmt;
use std::marker::PhantomData;

pub struct Context<'a> {
  phantom: PhantomData<&'a mut ()>,
  env: napi::Env,
}

impl fmt::Debug for Context<'_> {
  fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
    fm.debug_tuple("Context")
      .field(&self.env)
      .finish()
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

  pub fn external<T: 'static>(&self, data: T) -> NapiRef<'a, NapiExternal<T>> {
    NapiExternal::new(self.env, data).as_napi_ref()
  }

  pub fn function<'b, F, T>(&self, function: F) -> NapiRef<'a, NapiFunction>
  where
    'a: 'b,
    F: Fn(Context<'b>,
          NapiRef<'b, NapiValue>,
          Vec<NapiRef<'b, NapiValue>>
       ) -> NapiResult<'b, T> + 'static,
    T: NapiType + 'b
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