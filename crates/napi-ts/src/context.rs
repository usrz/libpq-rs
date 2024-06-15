use crate::contexts::*;
use crate::errors::*;
use crate::napi;
use crate::types::*;

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
