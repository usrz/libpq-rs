use crate::napi;
use crate::types::*;

use std::fmt::Debug;
use std::marker::PhantomData;
use crate::NapiResult;

pub struct Context<'a> {
  env: napi::Env<'a>,
}

impl Debug for Context<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Context")
      .field("@", &self.env.ptr()).finish()
  }
}

#[allow(private_bounds)]
impl <'a> Context<'a> {
  pub fn bigint<T: NapiInto<'a, NapiBigint<'a>>>(&self, value: T) -> NapiBigint<'a> {
    value.napi_into(self.env)
  }

  pub fn boolean<T: NapiInto<'a, NapiBoolean<'a>>>(&self, value: T) -> NapiBoolean<'a> {
    value.napi_into(self.env)
  }

  pub fn external<T: 'static>(&self, value: T) -> NapiExternal<'a, T> {
    value.napi_into(self.env)
  }

  pub fn function<F>(&self, function: F) -> NapiFunction<'a> where
    F: Fn(Context, NapiValue, Vec<NapiValue>) -> NapiResult + 'static
  {
    let value = NapiFunctionInternal { phantom: PhantomData, name: None, function };
    value.napi_into(self.env)
  }

  pub fn null(& self) -> NapiNull<'a> {
    ().napi_into(self.env)
  }

  pub fn number<T: NapiInto<'a, NapiNumber<'a>>>(&self, value: T) -> NapiNumber<'a> {
    value.napi_into(self.env)
  }

  pub fn object(&self) -> NapiObject<'a> {
    ().napi_into(self.env)
  }

  pub fn string<T: AsRef<str>>(&self, value: T) -> NapiString<'a> {
    value.as_ref().napi_into(self.env)
  }

  pub fn symbol<T: AsRef<str>>(&self, value: Option<T>) -> NapiSymbol<'a> {
    let symbol = NapiSymbolInternal::Symbol(match value {
      Some(str) => Some(str.as_ref().to_string()),
      None => None,
    });

    symbol.napi_into(self.env)
  }

  pub fn symbol_for<T: AsRef<str>>(&self, value: T) -> NapiSymbol<'a> {
    let symbol = NapiSymbolInternal::SymbolFor(value.as_ref().to_string());
    symbol.napi_into(self.env)
  }

  pub fn undefined(&self) -> NapiUndefined<'a> {
    ().napi_into(self.env)
  }
}

impl <'a> Context<'a> {
  pub (crate) fn new(env: napi::Env<'a>) -> Self {
    Self { env }
  }
}
