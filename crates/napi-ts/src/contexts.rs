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
  exports: napi::Handle,
}

impl fmt::Debug for InitContext<'_> {
  fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
    fm.debug_tuple("InitContext")
      .finish()
  }
}

impl <'a> NapiContext<'a> for InitContext<'a> {}

impl <'a> InitContext<'a> {
  pub (crate) fn new(exports: napi::Handle) -> Self {
    Self { phantom: PhantomData, exports }
  }

  pub fn exports(&self) -> NapiRef<'a, NapiObject<'a>> {
    unsafe { NapiObject::from_handle(self.exports).unwrap().as_napi_ref() }
  }
}

// ========================================================================== //
// FUNCTION CONTEXT                                                           //
// ========================================================================== //

pub struct FunctionContext<'a> {
  phantom: PhantomData<&'a mut ()>,
  this: napi::Handle,
  args: Vec<napi::Handle>,
}

impl fmt::Debug for FunctionContext<'_> {
  fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
    fm.debug_tuple("FunctionContext")
      .finish()
  }
}

impl <'a> NapiContext<'a> for FunctionContext<'a> {}

impl <'a> FunctionContext<'a> {
  pub (crate) fn new(this: napi::Handle, args: Vec<napi::Handle>) -> Self {
    Self { phantom: PhantomData, this, args: args.to_vec() }
  }

  pub fn this(&self) -> NapiRef<'a, NapiValue<'a>> {
    NapiValue::from_handle(self.this).as_napi_ref()
  }

  pub fn argc(&self) -> usize {
    self.args.len()
  }

  pub fn arg(&self, i: usize) -> NapiRef<'a, NapiValue<'a>> {
    NapiValue::from_handle(self.args[i]).as_napi_ref()
  }

  pub fn args(&self) -> Vec<NapiRef<'a, NapiValue<'a>>> {
    self.args
      .iter()
      .map(|handle| NapiValue::from_handle(*handle).as_napi_ref())
      .collect()
  }
}
