use crate::env::Napi;
use crate::errors::*;
use super::*;

use nodejs_sys::*;
use std::mem::MaybeUninit;
use std::os::raw;
use std::panic;
use std::ptr;

// ========================================================================== //
// INTERNAL WRAPPING                                                          //
// ========================================================================== //

struct CallbackWrapper {
  callback: Box<dyn Fn(Value, Vec<Value>) -> NapiResult<Value> + 'static>
}

impl CallbackWrapper {
  fn call(&self, this: Value, args: Vec<Value>) -> NapiResult<Value> {
    let callback = &self.callback;
    callback(this, args)
  }
}

extern "C" fn callback_trampoline(env: napi_env, info: napi_callback_info) -> Value {
  let env = Napi::new(env);

  // Call up our initialization function with exports wrapped in a NapiObject
  // and unwrap the result into a simple "napi_value" (the pointer)
  let panic = panic::catch_unwind(|| {
    let callback = get_cb_info(info);
    callback.call()
  });

  // See if the initialization panicked
  let result = panic.unwrap_or_else(|error| {
    Err(NapiError::from(format!("PANIC: {:?}", error)))
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

pub struct Callback {
  this: Value,
  args: Vec<Value>,
  wrapper: &'static CallbackWrapper,
}

impl Callback {
  pub fn call(self) -> NapiResult<Value> {
    self.wrapper.call(self.this, self.args)
  }
}

// ========================================================================== //

pub fn get_cb_info(info: CallbackInfo) -> Callback {
  unsafe {
    let mut argc = MaybeUninit::<usize>::zeroed();
    let mut this = MaybeUninit::<Value>::zeroed();
    let mut data = MaybeUninit::<*mut raw::c_void>::zeroed();

    // Figure out arguments count, "this" and our data (NapiCallbackWrapper)
    napi_check!(
      napi_get_cb_info,
      info,
      argc.as_mut_ptr(),
      ptr::null_mut(),
      this.as_mut_ptr(),
      data.as_mut_ptr()
    );

    // If we have arguments, extract them from our call info
    let args = match argc.assume_init() < 1 {
      true => vec![], // no args
      false => {
        let mut argv = vec![ptr::null_mut(); argc.assume_init()];
        napi_check!(
          napi_get_cb_info,
          info,
          argc.as_mut_ptr(),
          argv.as_mut_ptr(),
          ptr::null_mut(),
          ptr::null_mut()
        );
        argv
      }
    };

    // Build up our CallbacKData
    let pointer = data.assume_init() as *mut CallbackWrapper;
    let wrapper = &*{pointer};

    Callback { this: this.assume_init(), args, wrapper }
  }
}

pub fn create_function<F>(name: &str, callback: F) -> Value
where
  F: Fn(Value, Vec<Value>) -> NapiResult<Value> + Sized + 'static
{
  let wrapper = CallbackWrapper { callback: Box::new(callback) };
  let boxed = Box::new(wrapper);
  let pointer = Box::into_raw(boxed);

  // todo: finalize!

  // Shove everything in our wrapper...
  unsafe {
    let mut result = MaybeUninit::<Value>::zeroed();
    napi_check!(
      napi_create_function,
      name.as_ptr() as *const raw::c_char,
      name.len(),
      Some(callback_trampoline),
      pointer as *mut raw::c_void,
      result.as_mut_ptr()
    );
    result.assume_init()
  }
}

pub fn call_function(this: Value, function: Value, args: Vec<Value>) -> NapiResult<Value> {
  unsafe {
    let mut result = MaybeUninit::<Value>::zeroed();

    napi_call_function(
      Napi::env(),
      this,
      function,
      args.len(),
      args.as_ptr(),
      result.as_mut_ptr(),
    );

    if ! is_exception_pending() {
      return Ok(result.assume_init())
    }

    let mut error = MaybeUninit::<Value>::zeroed();
    napi_get_and_clear_last_exception(Napi::env(), error.as_mut_ptr());
    Err(error.assume_init().into())
  }
}
