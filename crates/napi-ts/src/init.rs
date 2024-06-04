pub use nodejs_sys;

use crate::env::Napi;
use crate::errors::NapiError;
use crate::errors::NapiResult;
use crate::napi;
use crate::types::*;

use nodejs_sys::*;
use std::ffi;
use std::panic;
use std::ptr;

pub fn napi_init(
  env: napi_env,
  exports: napi_value,
  init: fn(&Napi, NapiObject) -> NapiResult<NapiValues>
) -> napi_value {

  // Create a new "Napi" environment
  let napi = unsafe { Napi::new(env) };

  // Call up our initialization function with exports wrapped in a NapiObject
  // and unwrap the result into a simple "napi_value" (the pointer)
  let panic = panic::catch_unwind(|| {
    init(&napi, NapiObject::try_from(exports).unwrap())
      .map(|exports| unsafe { exports.as_napi_value() })
  });

  // See if the initialization panicked
  let result = panic.unwrap_or_else(|error| {
    match error.downcast_ref::<napi::NapiPanic>() {
      // This is a normal panic, we'll throw it later
      None => Err(NapiError::from(format!("{:?}", error))),
      // This is our "NAPI" panic, we have to handle it depending on status
      Some(napi_panic) => match napi_panic.status {
        // If the thread panicked because of an exception, well, it's thrown!
        napi::Status::napi_pending_exception => Ok(exports),
        // Any other status from Node should be handled...
        status => {
          let message = format!("Error calling \"{}\": {:?}", napi_panic.syscall, status);
          Err(NapiError::from(message))
        }
      }
    }
  });

  // When we get here, we dealt with possible panic situations, now we have
  // a result, which (if OK) will hold the napi_value to return to node or
  // (if ERR) will contain a NapiError to throw before returning
  if let Ok(exports) = result {
    return exports;
  }

  // Bad karma, we have to throw an error (properly)... If we can't we'll
  // kill the whole NodeJS process with a "fatal_error"!
  let message = result.unwrap_err().to_string();
  let mut error = message.to_owned();
  error.push('\0'); // make sure we're null terminated!!!

  unsafe {
    let status = nodejs_sys::napi_throw_error(
      env,
      ptr::null(),
      error.as_ptr() as *const ffi::c_char
    );

    match status {
      napi::Status::napi_ok => return exports,
      _ => {
        let location = format!("{} line {}", file!(), line!());
        let mmm = format!("Error throwing \"{}\" (status={:?})", message, status);
        nodejs_sys::napi_fatal_error(
          location.as_ptr() as *const ffi::c_char,
          location.len(),
          mmm.as_ptr() as *const ffi::c_char,
          mmm.len());
      }
    }
  }
}
