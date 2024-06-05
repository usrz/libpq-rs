use super::*;

use nodejs_sys::*;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::mem;
use std::os::raw;
use std::ptr;

// ========================================================================== //
// INTERNAL WRAPPING                                                          //
// ========================================================================== //

struct FinalizerWrapper<T> {
  phantom: PhantomData<T>
}

impl <T> FinalizerWrapper<T> {
  fn new() -> Self {
    Self { phantom: PhantomData }
  }

  fn finalize(self, data: *mut raw::c_void) {
    println!("Dropping {} @ {:?}", std::any::type_name::<T>(), data);
    let boxed = unsafe { Box::from_raw(data as *mut T) };
    drop(boxed);
  }
}

// ========================================================================== //
// TRAMPOLINE                                                                 //
// ========================================================================== //

extern "C" fn finalizer_trampoline<T>(_: napi_env, data: *mut raw::c_void, hint: *mut raw::c_void) {
  let finalizer = unsafe { Box::from_raw(hint as *mut FinalizerWrapper<T>) };
  finalizer.finalize(data);
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

pub fn add_finalizer<T>(value: Value, pointer: *mut T) {
  unsafe {
    // Build a wrapper for our trampoline and leak it as well
    let finalizer = FinalizerWrapper::<T>::new();
    let finalizer = Box::into_raw(Box::new(finalizer));

    // Get a hold on our trampoline's pointer (and erase its type!)
    let trampoline = finalizer_trampoline::<T>;
    let trampoline: FinalizerTrampoline = mem::transmute(trampoline as *mut ());

    napi_check!(
      napi_add_finalizer,
      value,
      pointer as *mut raw::c_void,
      Some(trampoline),
      finalizer as *mut raw::c_void,
      ptr::null_mut()
    );
  }
}

pub fn create_value_external<T>(data: *mut T) -> Value {
  unsafe {
    // Build a wrapper for our trampoline and leak it as well
    let finalizer = FinalizerWrapper::<T>::new();
    let finalizer = Box::into_raw(Box::new(finalizer));

    // Get a hold on our trampoline's pointer (and erase its type!)
    let trampoline = finalizer_trampoline::<T>;
    let trampoline: FinalizerTrampoline = mem::transmute(trampoline as *mut ());

    // Handle for our external data
    let mut result = MaybeUninit::<Value>::zeroed();

    // Create the
    napi_check!(
      napi_create_external,
      data as *mut raw::c_void,
      Some(trampoline),
      finalizer as *mut raw::c_void,
      result.as_mut_ptr()
    );
    result.assume_init()
  }
}

pub fn get_value_external(value: Value) -> *mut raw::c_void {
  unsafe {
    let mut result = MaybeUninit::<*mut raw::c_void>::zeroed();
    napi_check!(napi_get_value_external, value, result.as_mut_ptr());
    result.assume_init()
  }
}
