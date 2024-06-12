use crate::NapiErr;
use crate::NapiResult;
use std::fmt;
use std::marker::PhantomData;
use std::os::raw;
use std::panic::AssertUnwindSafe;
use std::panic;

mod errors;
mod externals;
mod functions;
mod objects;
mod primitives;

pub use nodejs_sys;

// =============================================================================

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypeOf {
  Undefined,
  Null,
  Boolean,
  Number,
  String,
  Symbol,
  Object,
  Function,
  External,
  Bigint,
}

impl From<nodejs_sys::napi_valuetype> for TypeOf {
  fn from(value: nodejs_sys::napi_valuetype) -> Self {
    match value {
      nodejs_sys::napi_valuetype::napi_undefined => Self::Undefined,
      nodejs_sys::napi_valuetype::napi_null => Self::Null,
      nodejs_sys::napi_valuetype::napi_boolean => Self::Boolean,
      nodejs_sys::napi_valuetype::napi_number => Self::Number,
      nodejs_sys::napi_valuetype::napi_string => Self::String,
      nodejs_sys::napi_valuetype::napi_symbol => Self::Symbol,
      nodejs_sys::napi_valuetype::napi_object => Self::Object,
      nodejs_sys::napi_valuetype::napi_function => Self::Function,
      nodejs_sys::napi_valuetype::napi_external => Self::External,
      nodejs_sys::napi_valuetype::napi_bigint => Self::Bigint,
      #[allow(unreachable_patterns)] // this should *really* never happen...
      _ => panic!("Unsupported JavaScript type \"{:?}\"", value)
    }
  }
}

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

  pub fn expect_type_of(&self, expected: TypeOf) -> Result<(), NapiErr> {
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
  pub (crate) fn handle(&self, value: nodejs_sys::napi_value) -> Handle<'a> {
    Handle { env: *self, value }
  }

  pub (crate) fn adopt(&self, handle: &Handle) -> Handle<'a> {
    assert!(self.env == handle.env.env, "Attempting to adopt foreign handle");
    Handle { env: *self, value: handle.value }
  }

  pub (crate) fn exec<F>(env: nodejs_sys::napi_env, callback: F) -> nodejs_sys::napi_value
  where
    F: Fn(Env) -> NapiResult
  {

    let env = Env { phantom: PhantomData, env };
    let callback = AssertUnwindSafe(callback);

    // Call up our initialization function with exports wrapped in a NapiObject
    // and unwrap the result into a simple "napi_value" (the pointer)
    let panic = panic::catch_unwind(|| {
      callback(env)
    });

    // See if the initialization panicked
    let result = panic.unwrap_or_else(|error| {
      if let Some(message) = error.downcast_ref::<&str>() {
        Err(format!("PANIC: {}", message).into())
      } else if let Some(message) = error.downcast_ref::<String>() {
        Err(format!("PANIC: {}", message).into())
      } else {
        Err("PANIC: Unknown error".into())
      }
    });

    // When we get here, we dealt with possible panic situations, now we have
    // a result, which (if OK) will hold the napi_value to return to node or
    // (if ERR) will contain a NapiError to throw before returning
    match result {
      Ok(exports) => exports.value,
      Err(error) => {
        error.throw(env);
        env.get_undefined().value()
      },
    }
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
