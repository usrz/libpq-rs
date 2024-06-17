use crate::napi::*;
use std::any::Any;

// ========================================================================== //
// TRAMPOLINE                                                                 //
// ========================================================================== //

/// Our finalizer trampoline, invoked by NodeJS and calling the `finalize()`
/// function on a [`Finalizable`].
///
extern "C" fn finalizer_trampoline<T: NapiFinalizable>(env: napi_env, data: *mut raw::c_void, _: *mut raw::c_void) {
  Env::exec(env, |env| unsafe {
    Box::from_raw(data as *mut T).finalize();
    Ok(env.get_undefined())
  });
}

// ========================================================================== //
// PUBLIC FACING                                                              //
// ========================================================================== //

impl Env {

  pub fn add_finalizer<T: NapiFinalizable>(&self, handle: &Handle, data: *mut T) {
    unsafe {
      // Get a hold on our trampoline's pointer
      let trampoline = finalizer_trampoline::<T>;

      env_check!(
        napi_add_finalizer,
        self,
        handle.0,
        data as *mut raw::c_void,
        Some(trampoline),
        ptr::null_mut(), // no hint
        ptr::null_mut() // no need for a napi_reference
      );
    }
  }

  pub fn create_value_external<T: NapiFinalizable>(&self, data: T) -> Handle {
    unsafe {
      // Box the data, and leak the raw pointer
      let boxed = Box::new(data);
      let pointer = Box::into_raw(boxed);

      // Get a hold on our trampoline's pointer
      let trampoline = finalizer_trampoline::<T>;

      // Value for our external data
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

      Handle(result.assume_init())
    }
  }

  pub fn get_value_external(&self, handle: &Handle) -> *mut dyn Any {
    unsafe {
      let mut result = MaybeUninit::<*mut raw::c_void>::zeroed();
      env_check!(
        napi_get_value_external,
        self,
        handle.0,
        result.as_mut_ptr()
      );
      result.assume_init()
    }
  }

  pub fn create_reference(&self, handle: &Handle) -> Reference {
    unsafe {
      let mut result = MaybeUninit::<napi_ref>::zeroed();
      env_check!(napi_create_reference, self, handle.0, 1, result.as_mut_ptr());
      Reference { value: result.assume_init() }
    }
  }

  pub fn get_reference_value(&self, reference: Reference) -> Handle {
    unsafe {
      let mut result = MaybeUninit::<napi_value>::zeroed();
      env_check!(napi_get_reference_value, self, reference.value, result.as_mut_ptr());
      Handle(result.assume_init())
    }
  }
}

impl Handle {
  pub fn add_finalizer<T: NapiFinalizable>(&self, data: *mut T) {
    env().add_finalizer(self, data)
  }

  pub fn get_value_external(&self) -> *mut dyn Any {
    env().get_value_external(self)
  }

  pub fn create_reference(&self) -> Reference {
    env().create_reference(self)
  }
}
