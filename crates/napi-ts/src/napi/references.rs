use super::*;

use nodejs_sys::*;
use std::mem::MaybeUninit;

pub fn create_reference(value: Value, initial: u32) -> Reference {
  unsafe {
    let mut result = MaybeUninit::<Reference>::zeroed();
    napi_check!(napi_create_reference, value, initial, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn delete_reference(reference: Reference) {
  unsafe { napi_check!(napi_delete_reference, reference) }
}

pub fn reference_ref(reference: Reference) -> u32 {
  unsafe {
    let mut result = MaybeUninit::<u32>::zeroed();
    napi_check!(napi_reference_ref, reference, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn reference_unref(reference: Reference) -> u32 {
  unsafe {
    let mut result = MaybeUninit::<u32>::zeroed();
    napi_check!(napi_reference_unref, reference, result.as_mut_ptr());
    result.assume_init()
  }
}
