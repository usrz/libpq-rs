use crate::errors::*;
use crate::napi;
use crate::types::*;

use std::panic;
use crate::context::MainContext;

pub fn register_module<'a, R: NapiType<'a> + Sized + 'a>(
  env: napi::Env,
  exports: napi::Handle,
  init: fn(MainContext<'a>, NapiObject<'a>) -> NapiResult<R>
) -> napi::Handle {

  // Create a new "Napi" environment

  // Call up our initialization function with exports wrapped in a NapiObject
  // and unwrap the result into a simple "napi_value" (the pointer)
  let panic = panic::catch_unwind(|| {
    let context = MainContext::new(env);
    let exports = NapiObject::from_napi(env, exports);
    init(context, exports)
      .map(|ret| -> napi::Handle { ret.napi_handle() })
  });

  // See if the initialization panicked
  let result = panic.unwrap_or_else(|error| {
    if let Some(message) = error.downcast_ref::<&str>() {
      Err(NapiError::from(format!("PANIC: {}", message)))
    } else if let Some(message) = error.downcast_ref::<String>() {
      Err(NapiError::from(format!("PANIC: {}", message)))
    } else {
      Err(NapiError::from("PANIC: Unknown error".to_owned()))
    }
  });

  // When we get here, we dealt with possible panic situations, now we have
  // a result, which (if OK) will hold the napi_value to return to node or
  // (if ERR) will contain a NapiError to throw before returning
  if let Err(error) = result {
    let throwable = napi::create_error(env, error.to_string());
    napi::throw(env, throwable); //TODO
  }

  // All done...
  // drop(napi);
  exports
}
