use crate::*;
use crate::napi;

pub trait NapiContext<'a> {
  fn array(&self) -> NapiRef<'a, NapiArray<'a>> {
    NapiArray::new().as_napi_ref()
  }

  fn bigint<V: Into<i128>>(&self, value: V) -> NapiRef<'a, NapiBigint<'a>> {
    NapiBigint::new(value.into()).as_napi_ref()
  }

  fn boolean<V: Into<bool>>(&self, value: V) -> NapiRef<'a, NapiBoolean<'a>> {
    NapiBoolean::new(value.into()).as_napi_ref()
  }

  fn external<T: 'static>(&self, data: T) -> NapiRef<'a, NapiExternal<'a, T>> {
    NapiExternal::new(data).as_napi_ref()
  }

  fn function<'b, F, R>(&self, function: F) -> NapiRef<'a, NapiFunction<'a>>
  where
    'a: 'b,
    F: (Fn(FunctionContext<'b>) -> NapiResult<'b, R>) + 'static,
    R: NapiType<'b> + 'b,
  {
    NapiFunction::new(None, function).as_napi_ref()
  }

  fn null(&self) -> NapiRef<'a, NapiNull<'a>> {
    NapiNull::new().as_napi_ref()
  }

  fn number<V: Into<f64>>(&self, value: V) -> NapiRef<'a, NapiNumber<'a>> {
    NapiNumber::new(value.into()).as_napi_ref()
  }

  fn object(&self) -> NapiRef<'a, NapiObject<'a>> {
    NapiObject::new().as_napi_ref()
  }

  fn string<T: AsRef<str>>(&self, value: T) -> NapiRef<'a, NapiString<'a>> {
    NapiString::new(value.as_ref()).as_napi_ref()
  }

  fn symbol<T: AsRef<str>>(&self, description: Option<T>) -> NapiRef<'a, NapiSymbol<'a>> {
    match description {
      Some(str) => NapiSymbol::new(Some(str.as_ref())),
      None => NapiSymbol::new(None),
    }.as_napi_ref()
  }

  fn symbol_for<T: AsRef<str>>(&self, description: T) -> NapiRef<'a, NapiSymbol<'a>> {
    NapiSymbol::new_for(description.as_ref()).as_napi_ref()
  }

  fn undefined(&self) -> NapiRef<'a, NapiUndefined<'a>> {
    NapiUndefined::new().as_napi_ref()
  }
}
