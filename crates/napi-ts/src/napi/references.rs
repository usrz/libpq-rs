use super::*;

use nodejs_sys::*;
use std::mem::MaybeUninit;

pub fn create_reference(env: Env, handle: Handle, initial: u32) -> Reference {
  unsafe {
    let mut result = MaybeUninit::<Reference>::zeroed();
    napi_check!(napi_create_reference, env, handle, initial, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn delete_reference(env: Env, reference: Reference) {
  unsafe { napi_check!(napi_delete_reference, env, reference) }
}

pub fn reference_ref(env: Env, reference: Reference) -> u32 {
  unsafe {
    let mut result = MaybeUninit::<u32>::zeroed();
    napi_check!(napi_reference_ref, env, reference, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn reference_unref(env: Env, reference: Reference) -> u32 {
  unsafe {
    let mut result = MaybeUninit::<u32>::zeroed();
    napi_check!(napi_reference_unref, env, reference, result.as_mut_ptr());
    result.assume_init()
  }
}
