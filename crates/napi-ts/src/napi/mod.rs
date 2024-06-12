use crate::*;
use std::fmt;
use std::marker::PhantomData;
use std::os::raw;
use std::panic::AssertUnwindSafe;
use std::panic;
use nodejs_sys::*;

mod errors;
mod externals;
mod functions;
mod objects;
mod primitives;

pub use nodejs_sys;

// =============================================================================

/// Wrap the concept of a _JavaScript Type_ as given to us by NodeJS.
///
/// See [`napi_valuetype`](https://nodejs.org/api/n-api.html#napi_valuetype)
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypeOf {
  /// The JavaScript constant `undefined`.
  Undefined,
  /// The JavaScript constant `null`.
  Null,
  /// The JavaScript type `boolean`.
  Boolean,
  /// The JavaScript type `number`.
  Number,
  /// The JavaScript type `string`.
  String,
  /// The JavaScript type `symbol`.
  Symbol,
  /// The JavaScript type `object`.
  Object,
  /// The JavaScript type `function`.
  Function,
  /// Indicates a native object provided to NodeJS.
  External,
  /// The JavaScript type `bigint`.
  Bigint,
}

impl From<napi_valuetype> for TypeOf {
  /// Create a [`TypeOf`] from a NodeJS [`napi_valuetype`].
  ///
  fn from(value: napi_valuetype) -> Self {
    match value {
      napi_valuetype::napi_undefined => Self::Undefined,
      napi_valuetype::napi_null => Self::Null,
      napi_valuetype::napi_boolean => Self::Boolean,
      napi_valuetype::napi_number => Self::Number,
      napi_valuetype::napi_string => Self::String,
      napi_valuetype::napi_symbol => Self::Symbol,
      napi_valuetype::napi_object => Self::Object,
      napi_valuetype::napi_function => Self::Function,
      napi_valuetype::napi_external => Self::External,
      napi_valuetype::napi_bigint => Self::Bigint,
      #[allow(unreachable_patterns)] // this should *really* never happen...
      _ => panic!("Unsupported JavaScript type \"{:?}\"", value)
    }
  }
}

// =============================================================================

/// A trait defining a callback from NodeJS indicating that the value
/// associated with this was garbage collected.
///
/// See [`napi_finalize`](https://nodejs.org/api/n-api.html#napi_finalize)
///
pub trait Finalizable {
  fn finalize(self);
}

// =============================================================================

/// A wrapper around NodeJS's own `napi_env`, _"a context that can be used to
/// persist VM-specific state"_.
///
/// See [`napi_env`](https://nodejs.org/api/n-api.html#napi_env)
///
#[derive(Clone, Copy)]
pub struct Env<'a> {
  phantom: PhantomData<&'a ()>,
  env: napi_env,
}

impl fmt::Debug for Env<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Env")
        .field("@", &self.env)
        .finish()
  }
}

impl <'a> Env<'a> {
  /// The pointer of the [`napi_env`], for debugging.
  ///
  pub (crate) fn ptr(&self) -> *mut () {
    self.env as *mut ()
  }

  /// Wrap a [`napi_value`] pointer and associate it with this [`Env`].
  ///
  pub (crate) fn handle(&self, value: napi_value) -> Handle<'a> {
    Handle { env: *self, value }
  }

  /// Execute a callback from NodeJS.
  ///
  /// This is our main entry point for all calls from NodeJS into Rust. It
  /// will take care of preparing an [`Env`] instance, and passing it to the
  /// callback.
  ///
  /// The callback's [`NapiResult`] will be used to either return a value to
  /// NodeJS or throw an exception. Also panics will be unwinded and thrown
  /// as JavaScript errors into the node environment.
  ///
  /// If an exception can not be thrown, the process will be terminated by
  /// [`napi_fatal_error`](https://nodejs.org/api/n-api.html#napi_fatal_error).
  ///
  pub (crate) fn exec<F>(env: napi_env, callback: F) -> napi_value
  where
    F: Fn(Env) -> NapiResult
  {
    // Create our Env and assert the callback to be unwind safe
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
        env.get_undefined().ptr()
      },
    }
  }
}

// =============================================================================

/// A wrapper around NodeJS's own `napi_value`, _"an opaque pointer that is
/// used to represent a JavaScript value."_.
///
/// See [`napi_value`](https://nodejs.org/api/n-api.html#napi_value)
///
#[derive(Clone, Copy)]
pub struct Handle<'a> {
  env: Env<'a>,
  value: napi_value,
}

impl fmt::Debug for Handle<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Handle")
        .field("@", &self.value)
        .finish()
  }
}

impl <'a> Handle<'a> {
  /// The pointer of the [`napi_env`], for debugging or converting to a
  /// [`NapiOk`] or [`NapiErr`].
  ///
  pub (crate) fn ptr(&self) -> napi_value {
    self.value
  }

  /// Return the [`Env`] associated with this [`Handle`].
  ///
  pub (crate) fn env(&self) -> Env<'a> {
    self.env
  }
}

// =============================================================================

// This doesn't seem to esist in "nodejs_sys"
extern "C" {
  fn node_api_symbol_for(
    env: napi_env,
    descr: *const raw::c_char,
    length: usize,
    result: *mut napi_value,
  ) -> napi_status;
}

/// Call a NodeJS API returning a status and check it's OK or panic.
macro_rules! env_check {
  ($syscall:ident, $self:ident, $($args:expr), +) => {
    match { $syscall($self.env, $($args),+) } {
      napi_status::napi_ok => (),
      status => panic!("Error calling \"{}\": {:?}", stringify!($syscall), status),
    }
  };
}

pub (self) use env_check;
