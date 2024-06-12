use crate::napi;
use crate::types::*;
use crate::context::Context;
use std::marker::PhantomData;

pub (crate) struct NapiFunctionInternal<'a, F>
where
  F: Fn(Context, NapiValue, Vec<NapiValue>) -> NapiResult + 'static
{
  pub (crate) phantom: PhantomData<&'a mut ()>,
  pub (crate) name: Option<String>,
  pub (crate) function: F
}

pub struct NapiFunction<'a> {
  handle: napi::Handle<'a>,
}

// ===== NAPI TYPE BASICS ======================================================

napi_type!(NapiFunction, Function);

impl <'a> TryFrom<NapiValue<'a>> for NapiFunction<'a> {
  type Error = NapiErr;

  fn try_from(value: NapiValue<'a>) -> Result<Self, Self::Error> {
    match value {
      NapiValue::Function(handle) => Ok(Self { handle }),
      _ => Err(format!("Can't downcast {} into NapiFunction", value).into()),
    }
  }
}

// ===== FUNCTION ==============================================================

impl <'a, F> NapiFrom<'a, NapiFunctionInternal<'a, F>> for NapiFunction<'a>
where
  F: Fn(Context, NapiValue, Vec<NapiValue>) -> NapiResult + 'static
{
  fn napi_from(function: NapiFunctionInternal<'a, F>, env: napi::Env<'a>) -> Self {
    let handle = env.create_function(function.name, move |env, this, args| {
      let env = Context::new(env);
      let this: NapiValue = this.into();
      let args: Vec<NapiValue> = args
        .iter()
        .map(|handle| (*handle).into())
        .collect();

      let foo = (function.function)(env, this, args);
      println!("{:?}", foo);
      foo
    });

    Self { handle }
  }
}
