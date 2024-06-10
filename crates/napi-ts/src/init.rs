use crate::errors::*;
use crate::napi;
use crate::types::*;

use std::panic;
use std::panic::AssertUnwindSafe;
use crate::context::MainContext;

pub fn register_module(
  env: napi::Env,
  exports: napi::Handle,
  init: fn(MainContext, NapiObject) -> NapiResult
) -> napi::Handle {
  println!("REGISTERING");

  let safe = AssertUnwindSafe(init);

  // Call up our initialization function with exports wrapped in a NapiObject
  // and unwrap the result into a simple "napi_value" (the pointer)
  let panic = panic::catch_unwind(|| {
    let handle = NapiHandle::from_napi(env, exports);

    safe(MainContext::new(env), NapiObject::from_napi_handle(handle)?)
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

  // When we get here, we dealt with possible panic situations, now we have
  // a result, which (if OK) will hold the napi_value to return to node or
  // (if ERR) will contain a NapiError to throw before returning
  match result {
    Ok(exports) => exports.value,
    Err(error) => {
      napi::throw(env, error.value.unwrap_or_else(|| {
        napi::create_error(env, error.to_string())
      }));
      // Just return the original exports unchanged... we'll throw anyhow!
      exports
    }
  }

  // println!("REGISTERING");

  // // All done...
  // // drop(napi);
  // exports
}
