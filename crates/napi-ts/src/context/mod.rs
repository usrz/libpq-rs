use crate::napi;
use crate::types::*;

use std::fmt::Debug;
use std::marker::PhantomData;

pub (crate) trait Env<'a>: Sized {
  fn napi_env(&self) -> napi::Env<'a>;
}

// ===========================================

#[allow(private_bounds)]
pub trait NapiEnv<'a>: Env<'a> + Sized {
  fn bigint<T: NapiInto<'a, NapiBigint<'a>>>(&self, value: T) -> NapiBigint<'a> {
    value.napi_into(self.napi_env())
  }

  fn boolean<T: NapiInto<'a, NapiBoolean<'a>>>(&self, value: T) -> NapiBoolean<'a> {
    value.napi_into(self.napi_env())
  }

  fn external<T: 'static>(&self, value: T) -> NapiExternal<'a, T> {
    value.napi_into(self.napi_env())
  }

  fn null(& self) -> NapiNull<'a> {
    ().napi_into(self.napi_env())
  }

  fn number<T: NapiInto<'a, NapiNumber<'a>>>(&self, value: T) -> NapiNumber<'a> {
    value.napi_into(self.napi_env())
  }

  fn object(&self) -> NapiObject<'a> {
    ().napi_into(self.napi_env())
  }

  fn string<T: AsRef<str>>(&self, value: T) -> NapiString<'a> {
    value.as_ref().napi_into(self.napi_env())
  }

  fn symbol<T: AsRef<str>>(&self, value: Option<T>) -> NapiSymbol<'a> {
    let symbol = Symbol::Symbol(match value {
      Some(str) => Some(str.as_ref().to_string()),
      None => None,
    });

    symbol.napi_into(self.napi_env())
  }

  fn symbol_for<T: AsRef<str>>(&self, value: T) -> NapiSymbol<'a> {
    let symbol = Symbol::SymbolFor(value.as_ref().to_string());
    symbol.napi_into(self.napi_env())
  }

  fn undefined(&self) -> NapiUndefined<'a> {
    ().napi_into(self.napi_env())
  }
}

// ===========================================

#[derive(Debug)]
pub struct InitEnv<'a> {
  phantom: PhantomData<&'a mut ()>,
  env: napi::Env<'a>,
}

impl <'a> Env<'a> for InitEnv<'a> {
  fn napi_env(&self) -> napi::Env<'a> {
    self.env
  }
}

impl <'a> NapiEnv<'a> for InitEnv<'a> {}

impl <'a> InitEnv<'a> {
  pub (crate) fn new(env: napi::Env<'a>) -> Self {
    Self { phantom: PhantomData, env }
  }
}
