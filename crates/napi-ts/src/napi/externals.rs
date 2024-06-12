use super::*;

use nodejs_sys::*;
use std::any::Any;
use std::mem::MaybeUninit;
use std::os::raw;
use std::ptr;

// ========================================================================== //
// TRAMPOLINE                                                                 //
// ========================================================================== //

/// Our finalizer trampoline, invoked by NodeJS and calling the `finalize()`
/// function on a [`Finalizable`].
///
extern "C" fn finalizer_trampoline<T: Finalizable>(env: napi_env, data: *mut raw::c_void, _: *mut raw::c_void) {
  Env::exec(env, |env| unsafe {
    Box::from_raw(data as *mut T).finalize();
    Ok(env.get_undefined().into())
  });
}

// ========================================================================== //
// PUBLIC FACING                                                              //
// ========================================================================== //

impl <'a> Env<'a> {

  pub fn add_finalizer<T: Finalizable>(&self, handle: &Handle, data: *mut T) {
    unsafe {
      // Get a hold on our trampoline's pointer
      let trampoline = finalizer_trampoline::<T>;

      env_check!(
        napi_add_finalizer,
        self,
        handle.value,
        data as *mut raw::c_void,
        Some(trampoline),
        ptr::null_mut(), // no hint
        ptr::null_mut() // no need for a napi_reference
      );
    }
  }

  pub fn create_value_external<T: Finalizable>(&self, data: T) -> Handle<'a> {
    unsafe {
      // Box the data, and leak the raw pointer
      let boxed = Box::new(data);
      let pointer = Box::into_raw(boxed);

      // Get a hold on our trampoline's pointer
      let trampoline = finalizer_trampoline::<T>;

      // Handle for our external data
      let mut result: MaybeUninit<napi_value> = MaybeUninit::zeroed();

      // Create the external
      env_check!(
        napi_create_external,
        self,
        pointer as *mut raw::c_void,
        Some(trampoline),
        ptr::null_mut(), // no hint
        result.as_mut_ptr()
      );

      Handle { env: *self, value: result.assume_init() }
    }
  }

  pub fn get_value_external(&self, handle: &Handle) -> *mut dyn Any {
    unsafe {
      let mut result = MaybeUninit::<*mut raw::c_void>::zeroed();
      env_check!(
        napi_get_value_external,
        self,
        handle.value,
        result.as_mut_ptr()
      );
      result.assume_init()
    }
  }

  pub fn create_reference(&self, handle: &Handle) -> Reference {
    unsafe {
      let mut result = MaybeUninit::<napi_ref>::zeroed();
      env_check!(napi_create_reference, self, handle.ptr(), 1, result.as_mut_ptr());
      Reference { value: result.assume_init() }
    }
  }

  pub fn get_reference_value(&self, reference: Reference) -> Handle<'a> {
    unsafe {
      let mut result = MaybeUninit::<napi_value>::zeroed();
      env_check!(napi_get_reference_value, self, reference.value, result.as_mut_ptr());
      self.handle(result.assume_init())
    }
  }
}

impl <'a> Handle<'a> {
  pub fn get_value_external(&self) -> *mut dyn Any {
    self.env.get_value_external(self)
  }

  pub fn create_reference(&self) -> Reference {
    self.env.create_reference(&self)
  }
}
