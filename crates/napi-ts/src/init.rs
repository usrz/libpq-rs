use crate::env::Napi;
use crate::errors::NapiError;
use crate::errors::NapiResult;
use crate::napi;
use crate::types::*;

use std::panic;

pub fn register_module(
  env: napi::Env,
  exports: napi::Value,
  init: fn(NapiObject) -> NapiResult<NapiReturn>
) -> napi::Value {

  // Create a new "Napi" environment
  let napi = Napi::new(env);

  // Call up our initialization function with exports wrapped in a NapiObject
  // and unwrap the result into a simple "napi_value" (the pointer)
  let panic = panic::catch_unwind(|| {
    let exports = NapiObject::from_napi_value(exports);
    init(exports).map(|exports| { exports.as_napi_value() })
  });

  // See if the initialization panicked
  let result = panic.unwrap_or_else(|error| {
    Err(NapiError::from(format!("PANIC: {:?}", error)))
  });

  // When we get here, we dealt with possible panic situations, now we have
  // a result, which (if OK) will hold the napi_value to return to node or
  // (if ERR) will contain a NapiError to throw before returning
  if let Err(error) = result {
    napi::throw(error.into());
  }

  // All done...
  drop(napi);
  exports
}
