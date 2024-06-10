use crate::napi;
use crate::types::*;

/// An internal trait providing the current [`napi::Env`]
pub (crate) trait Env: Sized {
  fn napi_env(&self) -> napi::Env;
}

// ===========================================

#[allow(private_bounds)]
pub trait NapiContext<'a>: Env + Sized
where  {
  fn bigint(&self, value: impl NapiInto<NapiBigint<'a>>) -> NapiBigint<'a> {
    value.napi_into(self.napi_env())
  }

  fn boolean(&self, value: impl NapiInto<NapiBoolean<'a>>) -> NapiBoolean<'a> {
    value.napi_into(self.napi_env())
  }

  fn external<T: 'static>(&self, value: T) -> NapiExternal<'a, T> {
    value.napi_into(self.napi_env())
  }

  fn null(&self) -> NapiNull<'a> {
    ().napi_into(self.napi_env())
  }

  fn number(&self, value: impl NapiInto<NapiNumber<'a>>) -> NapiNumber<'a> {
    value.napi_into(self.napi_env())
  }

  fn object(&self) -> NapiObject<'a> {
    ().napi_into(self.napi_env())
  }

  fn string<S: AsRef<str>>(&self, value: S) -> NapiString<'a> {
    value.as_ref().napi_into(self.napi_env())
  }

  fn symbol<S: AsRef<str>>(&self, value: Option<S>) -> NapiSymbol<'a> {
    match value {
      Some(desc) => Some(desc.as_ref()).napi_into(self.napi_env()),
      None => None.napi_into(self.napi_env()),
    }
  }

  fn undefined(&self) -> NapiUndefined<'a> {
    ().napi_into(self.napi_env())
  }
}
