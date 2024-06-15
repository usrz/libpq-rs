use crate::context::*;
use crate::napi;
use crate::types::*;
use std::fmt;
use std::marker::PhantomData;

// ========================================================================== //
// INIT CONTEXT                                                               //
// ========================================================================== //

pub struct InitContext<'a> {
  phantom: PhantomData<&'a mut ()>,
  env: napi::Env,
  exports: napi::Handle,
}

impl fmt::Debug for InitContext<'_> {
  fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
    fm.debug_tuple("InitContext")
      .field(&self.env)
      .finish()
  }
}

impl <'a> NapiContext<'a> for InitContext<'a> {}
impl <'a> NapiContextInternal<'a> for InitContext<'a> {
  #[inline]
  fn napi_env(&self) -> napi::Env {
    self.env
  }
}

impl <'a> InitContext<'a> {
  pub (crate) fn new(env: napi::Env, exports: napi::Handle) -> Self {
    Self { phantom: PhantomData, env, exports }
  }

  pub fn exports(&self) -> NapiRef<'a, NapiObject> {
    unsafe { NapiObject::from_handle(self.exports).as_napi_ref() }
  }
}

// ========================================================================== //
// FUNCTION CONTEXT                                                           //
// ========================================================================== //

pub struct FunctionContext<'a> {
  phantom: PhantomData<&'a mut ()>,
  env: napi::Env,
  this: napi::Handle,
  args: Vec<napi::Handle>,
}

impl fmt::Debug for FunctionContext<'_> {
  fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
    fm.debug_tuple("FunctionContext")
      .field(&self.env)
      .finish()
  }
}

impl <'a> NapiContext<'a> for FunctionContext<'a> {}
impl <'a> NapiContextInternal<'a> for FunctionContext<'a> {
  #[inline]
  fn napi_env(&self) -> napi::Env {
    self.env
  }
}

impl <'a> FunctionContext<'a> {
  pub (crate) fn new(env: napi::Env, this: napi::Handle, args: Vec<napi::Handle>) -> Self {
    Self { phantom: PhantomData, env, this, args: args.to_vec() }
  }

  pub fn this(&self) -> NapiRef<'a, NapiValue> {
    NapiValue::from_handle(self.this).as_napi_ref()
  }

  pub fn argc(&self) -> usize {
    self.args.len()
  }

  pub fn argv(&self, i: usize) -> NapiRef<'a, NapiValue> {
    NapiValue::from_handle(self.args[i]).as_napi_ref()
  }

  pub fn args(&self) -> Vec<NapiRef<'a, NapiValue>> {
    self.args
      .iter()
      .map(|handle| NapiValue::from_handle(*handle).as_napi_ref())
      .collect()
  }
}
