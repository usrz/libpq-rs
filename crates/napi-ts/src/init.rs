use crate::errors::*;
use crate::napi;
use crate::types::*;

use std::panic;
use std::panic::AssertUnwindSafe;
use crate::context::InitEnv;
use crate::context::Env;

pub fn register_module(
  env: nodejs_sys::napi_env,
  exports: nodejs_sys::napi_value,
  init: fn(InitEnv, NapiObject) -> NapiResult
) -> nodejs_sys::napi_value {

  let env = napi::Env::new(env);
  let safe = AssertUnwindSafe(init);

  // Call up our initialization function with exports wrapped in a NapiObject
  // and unwrap the result into a simple "napi_value" (the pointer)
  let panic = panic::catch_unwind(|| {
    let env = InitEnv::new(env);
    let handle = env.napi_env().handle(exports);
    let exports = NapiObject::from_napi_handle_unchecked(handle);
    safe(env, exports)
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
      let error = env.create_error(error.to_string());
      env.throw(&error);
      exports
    }
  }
}
