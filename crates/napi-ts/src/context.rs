use crate::napi;
use crate::types::*;
use std::fmt::Debug;
use std::marker::PhantomData;

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
  pub fn bigint<T: NapiInto<'a, NapiRef<'a, NapiBigint>>>(&self, value: T) -> NapiRef<'a, NapiBigint> {
    value.napi_into(self.env)
  }

  pub fn boolean<T: NapiInto<'a, NapiRef<'a, NapiBoolean>>>(&self, value: T) -> NapiRef<'a, NapiBoolean> {
    value.napi_into(self.env)
  }

  // pub fn external<T: 'static>(&self, value: T) -> NapiExternal<'a, T> {
  //   value.napi_into(self.env)
  // }

  // pub fn function<F>(&self, function: F) -> NapiFunction<'a> where
  //   F: Fn(Context, NapiValue, Vec<NapiValue>) -> NapiResult + 'static
  // {
  //   let value = NapiFunctionInternal { phantom: PhantomData, name: None, function };
  //   value.napi_into(self.env)
  // }

  pub fn null(&self) -> NapiRef<'a, NapiNull> {
    ().napi_into(self.env)
  }

  pub fn number<T: NapiInto<'a, NapiRef<'a, NapiNumber>>>(&self, value: T) -> NapiRef<'a, NapiNumber> {
    value.napi_into(self.env)
  }

  pub fn object(&self) -> NapiRef<'a, NapiObject> {
    ().napi_into(self.env)
  }

  pub fn string<T: AsRef<str>>(&self, value: T) -> NapiRef<'a, NapiString> {
    value.as_ref().napi_into(self.env)
  }

  pub fn symbol<T: AsRef<str>>(&self, value: Option<T>) -> NapiRef<'a, NapiSymbol> {
    let symbol = NapiSymbolInternal::Symbol(match value {
      Some(str) => Some(str.as_ref().to_string()),
      None => None,
    });

    symbol.napi_into(self.env)
  }

  pub fn symbol_for<T: AsRef<str>>(&self, value: T) -> NapiRef<'a, NapiSymbol> {
    let symbol = NapiSymbolInternal::SymbolFor(value.as_ref().to_string());
    symbol.napi_into(self.env)
  }

  pub fn undefined(&self) -> NapiRef<'a, NapiUndefined> {
    ().napi_into(self.env)
  }
}

impl <'a> Context<'a> {
  pub (crate) fn new(env: napi::Env) -> Self {
    Self { phantom: PhantomData, env }
  }
}
