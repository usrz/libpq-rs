use super::*;

use nodejs_sys::*;
use std::mem::MaybeUninit;
use std::mem;
use std::os::raw;
use std::any::Any;
use std::ptr;

// ========================================================================== //
// TRAMPOLINE                                                                 //
// ========================================================================== //

extern "C" fn finalizer_trampoline<T: Finalizable>(_: napi_env, data: *mut raw::c_void, _: *mut raw::c_void) {
  unsafe { Box::from_raw(data as *mut T) }.finalize();
}

type FinalizerTrampoline =
  unsafe extern "C" fn(
    env: napi_env,
    finalize_data: *mut raw::c_void,
    finalize_hint: *mut raw::c_void,
  );

// ========================================================================== //
// PUBLIC FACING                                                              //
// ========================================================================== //

pub fn add_finalizer<T: Finalizable>(value: Value, data: *mut T) {
  unsafe {
    // Get a hold on our trampoline's pointer (and erase its type!)
    let trampoline = finalizer_trampoline::<T>;
    let trampoline: FinalizerTrampoline = mem::transmute(trampoline as *mut ());

    napi_check!(
      napi_add_finalizer,
      value,
      data as *mut raw::c_void,
      Some(trampoline),
      ptr::null_mut(), // no hint
      ptr::null_mut() // no need for a napi_reference
    );
  }
}

pub fn create_value_external<T: Finalizable>(data: T) -> Value {
  unsafe {
    // Box the data, and leak the raw pointer
    let boxed = Box::new(data);
    let pointer = Box::into_raw(boxed);

    // Get a hold on our trampoline's pointer (and erase its type!)
    let trampoline = finalizer_trampoline::<T>;
    let trampoline: FinalizerTrampoline = mem::transmute(trampoline as *mut ());

    // Handle for our external data
    let mut result = MaybeUninit::<Value>::zeroed();

    // Create the external
    napi_check!(
      napi_create_external,
      pointer as *mut raw::c_void,
      Some(trampoline),
      ptr::null_mut(), // no hint
      result.as_mut_ptr()
    );

    result.assume_init()
  }
}

pub fn get_value_external(value: Value) -> *mut dyn Any {
  unsafe {
    let mut result = MaybeUninit::<*mut raw::c_void>::zeroed();
    napi_check!(napi_get_value_external, value, result.as_mut_ptr());
    result.assume_init()
  }
}
