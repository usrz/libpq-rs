use crate::NapiFinalizable;
use crate::NapiTypeOf;
use crate::errors::*;
use nodejs_sys::*;
use std::cell::Cell;
use std::fmt;
use std::mem::MaybeUninit;
use std::os::raw;
use std::panic;
use std::ptr;

mod arrays;
mod errors;
mod externals;
mod macros;
mod functions;
mod objects;
mod primitives;

pub use nodejs_sys;

pub (self) use macros::env_check;

// =============================================================================

thread_local! {
  static NAPI_ENV: Cell<napi_env> = Cell::new(ptr::null_mut());
}

// =============================================================================

/// A wrapper around NodeJS's own `napi_env`, _"a context that can be used to
/// persist VM-specific state"_.
///
/// See [`napi_env`](https://nodejs.org/api/n-api.html#napi_env)
///
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Env(pub napi_env);

impl fmt::Debug for Env {
  fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
      fm.debug_tuple("Env")
        .field(&self.0)
        .finish()
  }
}

impl From<napi_env> for Env {
  fn from(value: napi_env) -> Self {
    Self(value)
  }
}

impl Env {
  /// Return an [`Env`] bound to this thread _(a thread local)_.
  ///
  fn local() -> Env {
    let env = NAPI_ENV.get();
    assert_ne!(env, ptr::null_mut(), "NAPI Environment not bound to current thread");
    Env(env)
  }

  pub (crate) fn handle(&self, value: napi_value) -> Handle {
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
    F: Fn(Env) -> Result<Handle, NapiErr>
  {
    // Contextualize ourselves in the *current* thread... There can only
    // be one "napi_env" at a time in a single thread, and since we're supposed
    // to be fully reentrant, this shouldn't create any problems...
    // This allows to have nested calls Node->Rust->Node->Rust ... without fail.
    let old = NAPI_ENV.replace(env);
    println!(">>> ENTER >>> old={:?} new={:?}", old, env);

    // Create our Env and assert the callback to be unwind safe
    let env = Env(env);
    let callback = panic::AssertUnwindSafe(callback);

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

    // Return the old "napi_env" into the thread local.
    println!(">>> EXIT >>> new={:?} old={:?} ", env, old);
    NAPI_ENV.set(old);

    // When we get here, we dealt with possible panic situations, now we have
    // a result, which (if OK) will hold the `Handle` with the `napi_value`
    // to return to node or (if ERR) will hold a `NapiErr` containing either a
    // `Handle` to throw, or a message from which to generate an error to throw.
    result.unwrap_or_else(|error| {
      env.throw(&error.into_handle(env))
    }).value
  }
}

// =============================================================================

/// A wrapper around NodeJS's own `napi_value`, _"an opaque pointer that is
/// used to represent a JavaScript value."_.
///
/// This always associates the `napi_value` with the [`Env`] it lives into.
///
/// See [`napi_value`](https://nodejs.org/api/n-api.html#napi_value)
///
#[derive(Clone, Copy)]
pub struct Handle {
  env: Env,
  value: napi_value,
}

impl fmt::Debug for Handle {
  fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
    fm.debug_tuple("Handle")
      .field(&self.value)
      .finish()
  }
}

impl Handle {
  pub fn env(&self) -> Env {
    self.env
  }
}

// =============================================================================

pub struct Reference {
  value: napi_ref,
}

impl fmt::Debug for Reference {
  fn fmt(&self, fm: &mut fmt::Formatter<'_>) -> fmt::Result {
    fm.debug_tuple("Reference")
      .field(&self.value)
      .finish()
  }
}

impl Clone for Reference {
  fn clone(&self) -> Self {
    let env = Env::local();
    unsafe { env_check!(napi_reference_ref, env, self.value, ptr::null_mut()) };
    Self { value: self.value }
  }
}

impl Drop for Reference {
  fn drop(&mut self) {
    let env = Env::local();

    unsafe {
      let mut result = MaybeUninit::<u32>::zeroed();
      env_check!(napi_reference_unref, env, self.value, result.as_mut_ptr());
      let count = result.assume_init();

      if count == 0 {
        env_check!(napi_delete_reference, env, self.value);
      }
    };
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
