use core::fmt;
use std::marker::PhantomData;
use std::os::raw;

mod errors;
mod externals;
mod objects;
mod primitives;

pub use nodejs_sys;
pub type CallbackInfo = nodejs_sys::napi_callback_info;
pub type Reference = nodejs_sys::napi_ref;
pub type TypeOf = nodejs_sys::napi_valuetype;

// =============================================================================

pub trait Finalizable {
  fn finalize(self);
}

// =============================================================================

#[derive(Clone, Copy)]
pub struct Handle<'a> {
  env: Env<'a>,
  value: nodejs_sys::napi_value,
}

impl fmt::Debug for Handle<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Handle")
        .field("@", &self.value)
        .finish()
  }
}

impl <'a> Handle<'a> {
  pub fn env(&self) -> Env<'a> {
    self.env
  }

  pub fn type_of(&self) -> TypeOf {
    self.env.type_of(self)
  }

  pub fn expect_type_of(& self, expected: TypeOf) -> Result<(), NapiErr> {
    let actual = self.type_of();
    match actual == expected {
      false => Err(format!("Expected type {:?}, actual {:?}", expected, actual).into()),
      true => Ok(())
    }
  }

  pub (crate) fn value(&self) -> nodejs_sys::napi_value {
    self.value
  }
}

// =============================================================================

#[derive(Clone, Copy)]
pub struct Env<'a> {
  phantom: PhantomData<&'a ()>,
  env: nodejs_sys::napi_env,
}

impl fmt::Debug for Env<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Env")
        .field("@", &self.env)
        .finish()
  }
}

impl <'a> Env<'a> {
  pub (crate) fn new(env: nodejs_sys::napi_env) -> Self {
    Self { phantom: PhantomData, env }
  }

  pub (crate) fn handle(&self, value: nodejs_sys::napi_value) -> Handle<'a> {
    Handle { env: *self, value }
  }

  pub (crate) fn adopt(&self, handle: &Handle) -> Handle<'a> {
    assert!(self.env == handle.env.env, "Attempting to adopt foreign handle");
    Handle { env: *self, value: handle.value }
  }
}

// =============================================================================

// this doesn't seem to esist in "nodejs_sys"
extern "C" {
  fn node_api_symbol_for(
    env: nodejs_sys::napi_env,
    descr: *const raw::c_char,
    length: usize,
    result: *mut nodejs_sys::napi_value,
  ) -> nodejs_sys::napi_status;
}

/// Call a NodeJS API returning a status and check it's OK or panic.
macro_rules! env_check {
  ($syscall:ident, $self:ident, $($args:expr), +) => {
    match { $syscall($self.env, $($args),+) } {
      nodejs_sys::napi_status::napi_ok => (),
      status => panic!("Error calling \"{}\": {:?}", stringify!($syscall), status),
    }
  };
}

pub (self) use env_check;
use crate::NapiErr;
