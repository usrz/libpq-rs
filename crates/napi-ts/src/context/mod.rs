use crate::NapiBigint;
use std::marker::PhantomData;
use crate::napi;
use std::fmt::Debug;
use crate::NapiInto;
use crate::NapiBoolean;
use crate::NapiNull;
use crate::NapiNumber;
use crate::NapiUndefined;
use crate::NapiString;
use crate::NapiObject;
use crate::NapiSymbol;
use crate::NapiFrom;
use crate::NapiValue;

/// An internal trait providing the current [`napi::Env`]
pub (crate) trait Env: Sized {
  fn napi_env(&self) -> napi::Env;
}

// ===========================================

#[allow(private_bounds)]
pub trait NapiContext<'a>: Env + Sized
where  {
  fn bigint(&mut self, value: impl NapiInto<NapiBigint<'a>>) -> NapiBigint<'a> {
    value.napi_into(self.napi_env())
  }

  fn boolean(&mut self, value: impl NapiInto<NapiBoolean<'a>>) -> NapiBoolean<'a> {
    value.napi_into(self.napi_env())
  }

  fn null(&mut self) -> NapiNull<'a> {
    ().napi_into(self.napi_env())
  }

  fn number(&mut self, value: impl NapiInto<NapiNumber<'a>>) -> NapiNumber<'a> {
    value.napi_into(self.napi_env())
  }

  fn object(&mut self) -> NapiObject<'a> {
    ().napi_into(self.napi_env())
  }

  fn string<S: AsRef<str>>(&mut self, value: S) -> NapiString<'a> {
    value.as_ref().napi_into(self.napi_env())
  }

  fn symbol<S: AsRef<str>>(&mut self, value: Option<S>) -> NapiSymbol<'a> {
    match value {
      Some(desc) => Some(desc.as_ref()).napi_into(self.napi_env()),
      None => None.napi_into(self.napi_env()),
    }
  }

  fn undefined(&mut self) -> NapiUndefined<'a> {
    ().napi_into(self.napi_env())
  }
}

// ===========================================

#[derive(Debug)]
pub struct MainContext<'a> {
  phantom: PhantomData<&'a mut ()>,
  env: napi::Env,
}

impl <'a> NapiContext<'a> for MainContext<'a> {}

impl <'a> Env for MainContext<'_> {
  fn napi_env(&self) -> napi::Env {
    self.env
  }
}

impl MainContext<'_> {
  pub (crate) fn new(env: napi::Env) -> Self {
    Self { phantom: PhantomData, env }
  }
}



fn foo(mut env: MainContext) {
  let bar = env.null(); //string("sugar");
  let baz: NapiValue = bar.into();

  Blurb::call(move || {
    // println!("{:?}", baz);
  });
}


struct Blurb {}

impl Blurb {
  pub fn call<F>(callback: F) -> Self
  where
    F: Fn() + 'static,
  {
    // Self::named("", callback)
    callback();
    Self {}
  }
}
