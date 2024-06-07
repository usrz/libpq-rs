use crate::env::Napi;
use crate::types::NapiValue;
use crate::errors::*;
use super::*;

use nodejs_sys::*;
use std::mem::MaybeUninit;
use std::os::raw;
use std::panic;
use std::ptr;
use std::mem;
use std::any::TypeId;
use std::any::type_name;

// ========================================================================== //
// INTERNAL WRAPPING                                                          //
// ========================================================================== //

/// The [`CallbackWrapper`] is what gets stored into memory (and passed onto)
/// NodeJS by [`create_function`].
///
/// When our trampoline gets invoked because of a callback from NodeJS to Rust,
/// the trampoline will *first* call [`get_cb_info`]. In the _data_ part of
/// retrieved we'll find a pointer to this structure...
struct CallbackWrapper<F>
where
  F: Fn(Value, Vec<Value>) -> NapiResult<Value> + Sized + 'static
{
  type_id: TypeId,
  function: *mut F,
}

impl <F> CallbackWrapper<F>
where
  F: Fn(Value, Vec<Value>) -> NapiResult<Value> + Sized + 'static
{
  fn call(&self, this: Value, args: Vec<Value>) -> NapiResult<Value> {
    let cb = unsafe { &* { self.function }};
    cb(this, args)
  }
}

impl <F> Finalizable for CallbackWrapper<F>
where
  F: Fn(Value, Vec<Value>) -> NapiResult<Value> + Sized + 'static
{
  fn finalize(self) {
    drop(unsafe { Box::from_raw(self.function) });
  }
}

// ========================================================================== //
// TRAMPOLINE                                                                 //
// ========================================================================== //

extern "C" fn callback_trampoline<F>(env: napi_env, info: napi_callback_info) -> Value
where
  F: Fn(Value, Vec<Value>) -> NapiResult<Value> + Sized + 'static
{
  let env = Napi::new(env);

  // Call up our initialization function with exports wrapped in a NapiObject
  // and unwrap the result into a simple "napi_value" (the pointer)
  let panic = panic::catch_unwind(|| {
    let callback = get_cb_info::<F>(info);
    callback.call()
  });

  // See if the initialization panicked
  let result = panic.unwrap_or_else(|error| {
    if let Some(message) = error.downcast_ref::<&str>() {
      Err(NapiError::from(format!("PANIC: {}", message)))
    } else if let Some(message) = error.downcast_ref::<String>() {
      Err(NapiError::from(format!("PANIC: {}", message)))
    } else {
      Err(NapiError::from("PANIC: Unknown error".to_owned()))
    }
  });

  // When we get here, we dealt with possible panic situations, now we have
  // a result, which (if OK) will hold the napi_value to return to node or
  // (if ERR) will contain a NapiError to throw before returning
  if let Err(error) = result {
    throw(error.into());
  }

  // All done...
  drop(env);
  return ptr::null_mut()
}

// ========================================================================== //
// PUBLIC FACING                                                              //
// ========================================================================== //


pub struct Callback<F>
where
  F: Fn(Value, Vec<Value>) -> NapiResult<Value> + Sized + 'static
{
  this: Value,
  args: Vec<Value>,
  wrapper: &'static CallbackWrapper<F>,
}

impl <F> Callback<F>
where
  F: Fn(Value, Vec<Value>) -> NapiResult<Value> + Sized + 'static
{
  pub fn call(self) -> NapiResult<Value> {
    self.wrapper.call(self.this, self.args)
  }
}

// ========================================================================== //

pub fn get_cb_info<F>(info: CallbackInfo) -> Callback<F>
where
  F: Fn(Value, Vec<Value>) -> NapiResult<Value> + Sized + 'static
{
  unsafe {
    let mut argc = MaybeUninit::<usize>::zeroed();
    let mut this = MaybeUninit::<Value>::zeroed();
    let mut data = MaybeUninit::<*mut raw::c_void>::zeroed();

    // Figure out arguments count, "this" and our data (NapiCallbackWrapper)
    napi_check!(
      napi_get_cb_info,
      info,
      argc.as_mut_ptr(), // number of arguments in the call
      ptr::null_mut(), // we'll read arguments later
      this.as_mut_ptr(), // the "this" value of the called function
      data.as_mut_ptr() // opaque pointer that *should* point to our wrapper
    );

    // If we have arguments, extract them from our call info
    let args = match argc.assume_init() < 1 {
      true => vec![], // no args
      false => {
        let mut argv = vec![ptr::null_mut(); argc.assume_init()];
        napi_check!(
          napi_get_cb_info,
          info,
          argc.as_mut_ptr(), // nuber of arguments to read
          argv.as_mut_ptr(), // pointer to the *actual* arguments
          ptr::null_mut(), // we got our "this" before
          ptr::null_mut() // we got our callback wrapper before
        );
        argv
      }
    };

    // Build up our CallbackWrapper from the data pointer
    let pointer = data.assume_init() as *mut CallbackWrapper<F>;
    let wrapper = &*{pointer};

    // Triple check that the type IDs of what's in memory, and of what we're
    // being called on match, if so, good, otherwise panic
    match TypeId::of::<F>() == wrapper.type_id {
      false => panic!("Mismatched type id in callback from function {}", type_name::<F>()),
      true => Callback { this: this.assume_init(), args, wrapper },
    }
  }
}

type CallbackTrampoline = unsafe extern "C" fn(env: napi_env, info: napi_callback_info) -> napi_value;

pub fn create_function<F>(name: &str, function: F) -> Value
where
  F: Fn(Value, Vec<Value>) -> NapiResult<Value> + Sized + 'static
{
  // Box up the callback function and immediately leak it
  let boxed_function = Box::new(function);
  let pointer_function = Box::into_raw(boxed_function);

  // Get a hold on our trampoline's pointer (and erase its type!)
  let trampoline = callback_trampoline::<F>;
  let trampoline: CallbackTrampoline = unsafe { mem::transmute(trampoline as *mut ()) };

  // Create a callback wrapper with some safety for types
  let wrapper = CallbackWrapper {
    function: pointer_function,
    type_id: TypeId::of::<F>(),
  };

  // Once again box-and-leak the wrapper... This will be the *data* passed
  // in the CallbackInfo structure when we get called...
  let boxed_wrapper = Box::new(wrapper);
  let pointer_wrapper = Box::into_raw(boxed_wrapper);

  // Send everything off to NodeJS...
  unsafe {
    let mut result = MaybeUninit::<Value>::zeroed();
    napi_check!(
      napi_create_function,
      name.as_ptr() as *const raw::c_char,
      name.len(),
      Some(trampoline),
      pointer_wrapper as *mut raw::c_void,
      result.as_mut_ptr()
    );

    // Get the "napi_value" that NodeJS returned
    let value = result.assume_init();

    // Add a finalizer that will drop *both* wrapper and function (it can be
    // a closure, it may have variables moved to it)
    add_finalizer(value, pointer_wrapper);

    // Done and return the value
    value
  }
}

pub fn call_function(this: Value, function: Value, args: Vec<Value>) -> NapiResult<Value> {
  unsafe {
    let mut result = MaybeUninit::<Value>::zeroed();

    // Call the function
    napi_call_function(
      Napi::env(),
      this,
      function,
      args.len(),
      args.as_ptr(),
      result.as_mut_ptr(),
    );

    // If there's no pending exception fron NodeJS, then all is good
    if ! is_exception_pending() {
      return Ok(result.assume_init())
    }

    // There's a pending exception, wrap into a NapiError and err the result
    let mut error = MaybeUninit::<Value>::zeroed();
    napi_get_and_clear_last_exception(Napi::env(), error.as_mut_ptr());
    Err(NapiValue::from(error.assume_init()).into())
  }
}
