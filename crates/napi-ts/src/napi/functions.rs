use crate::errors::*;
use super::*;

use nodejs_sys::*;
use std::any::TypeId;
use std::any::type_name;
use std::mem::MaybeUninit;
use std::mem;
use std::os::raw;
use std::panic;
use std::ptr;

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
  F: Fn(Env, Handle, Vec<Handle>) -> Result<Handle, NapiErr> + 'static
{
  type_id: TypeId,
  function: *mut F,
}

impl <F> CallbackWrapper<F>
where
  F: Fn(Env, Handle, Vec<Handle>) -> Result<Handle, NapiErr> + 'static
{
  fn call(&self, env: Env, this: Handle, args: Vec<Handle>) -> Result<Handle, NapiErr> {
    let cb = unsafe { &* { self.function }};
    cb(env, this, args)
  }
}

impl <F> Finalizable for CallbackWrapper<F>
where
  F: Fn(Env, Handle, Vec<Handle>) -> Result<Handle, NapiErr> + 'static
{
  fn finalize(self) {
    drop(unsafe { Box::from_raw(self.function) });
  }
}

// ========================================================================== //
// TRAMPOLINE                                                                 //
// ========================================================================== //

extern "C" fn callback_trampoline<F>(env: napi_env, info: napi_callback_info) -> napi_value
where
  F: Fn(Env, Handle, Vec<Handle>) -> Result<Handle, NapiErr> + 'static
{
  Env::exec(env, |env| unsafe {
    let mut argc = MaybeUninit::<usize>::zeroed();
    let mut this = MaybeUninit::<napi_value>::zeroed();
    let mut data = MaybeUninit::<*mut raw::c_void>::zeroed();

    // Figure out arguments count, "this" and our data (NapiCallbackWrapper)
    env_check!(
      napi_get_cb_info,
      env,
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
        env_check!(
          napi_get_cb_info,
          env,
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
    if TypeId::of::<F>() != wrapper.type_id {
      return Err(format!("Mismatched type id in callback {}", type_name::<F>()).into())
    }

    let this = env.handle(this.assume_init());
    let args: Vec<Handle> = args
        .into_iter()
        .map(|value| env.handle(value))
        .collect();

    wrapper.call(env, this, args)
  })
}

// ========================================================================== //
// PUBLIC FACING                                                              //
// ========================================================================== //

impl Env {
  pub fn create_function<F>(&self, name: Option<&str>, function: F) -> Handle
  where
    F: Fn(Env, Handle, Vec<Handle>) -> Result<Handle, NapiErr> + 'static
  {
    // See if this function is named or anonymous
    let (name, name_len) = match name {
      Some(name) => (name.as_ptr(), name.len()),
      None => (ptr::null(), 0),
    };

    // Box up the callback function and immediately leak it
    let boxed_function = Box::new(function);
    let pointer_function = Box::into_raw(boxed_function);

    // Create a callback wrapper with some safety for types
    let wrapper = CallbackWrapper::<F> {
      function: pointer_function,
      type_id: TypeId::of::<F>(),
    };

    // Once again box-and-leak the wrapper... This will be the *data* passed
    // in the CallbackInfo structure when we get called...
    let boxed_wrapper = Box::new(wrapper);
    let pointer_wrapper = Box::into_raw(boxed_wrapper);

    // Get a hold on our trampoline's pointer (and erase its type!)
    let trampoline = callback_trampoline::<F>;
    let trampoline: napi_callback = unsafe { mem::transmute(trampoline as *mut ()) };

    // Send everything off to NodeJS...
    unsafe {
      let mut result = MaybeUninit::<napi_value>::zeroed();
      env_check!(
        napi_create_function,
        self,
        name as *const raw::c_char,
        name_len,
        trampoline,
        pointer_wrapper as *mut raw::c_void,
        result.as_mut_ptr()
      );

      // Get the "napi_value" that NodeJS returned
      let handle = self.handle(result.assume_init());

      // Add a finalizer that will drop *both* wrapper and function (it can be
      // a closure, it may have variables moved to it)
      self.add_finalizer(&handle, pointer_wrapper);

      // Done and return the value
      handle
    }
  }

  pub fn call_function(&self, function: &Handle, this: &Handle, args: Vec<&Handle>) -> Result<Handle, Handle> {
    unsafe {
      let mut result = MaybeUninit::<napi_value>::zeroed();

      let args: Vec<napi_value> = args
          .into_iter()
          .map(|arg| arg.value)
          .collect::<Vec<napi_value>>();

      // Call the function
      env_check!(
        napi_call_function,
        self,
        this.value,
        function.value,
        args.len(),
        args.as_ptr(),
        result.as_mut_ptr()
      );

      // If there's no pending exception fron NodeJS, then all is good
      if ! self.is_exception_pending() {
        return Ok(self.handle(result.assume_init()))
      }

      // There's a pending exception, wrap into a NapiError and err the result
      let mut error = MaybeUninit::<napi_value>::zeroed();

      env_check!(
        napi_get_and_clear_last_exception,
        self,
        error.as_mut_ptr()
      );

      Err(self.handle(error.assume_init()))
    }
  }
}

impl Handle {
  pub fn call_function(&self, this: &Handle, args: Vec<&Handle>) -> Result<Handle, Handle> {
    self.env.call_function(self, this, args)
  }
}
