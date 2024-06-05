// pub use nodejs_sys;

use crate::env::Napi;
use crate::errors::*;

use nodejs_sys::*;
use std::mem::MaybeUninit;
use std::panic;
use std::ptr;
use std::os::raw;
use std::ptr::null_mut;

pub type CallbackInfo = nodejs_sys::napi_callback_info;
pub type Env = nodejs_sys::napi_env;
pub type Reference = nodejs_sys::napi_ref;
pub type Status = nodejs_sys::napi_status;
pub type Value = nodejs_sys::napi_value;
pub type ValueType = nodejs_sys::napi_valuetype;

// ========================================================================== //
// PANIC! When NodeJS' API fails we *panic* with a nice error message         //
// ========================================================================== //

/// Call a NodeJS API returning a status and check it's OK or panic.
macro_rules! napi_check {
  ($syscall:ident, $($args:expr), +) => {
    match { $syscall(Napi::env(), $($args),+) } {
      Status::napi_ok => (),
      status => panic!("Error calling \"{}\": {:?}", stringify!($syscall), status),
    }
  };
}

// ========================================================================== //
// ERRORS RELATED                                                             //
// ========================================================================== //

pub fn is_exception_pending() -> bool {
  unsafe {
    let mut result = MaybeUninit::<bool>::zeroed();
    napi_check!(napi_is_exception_pending, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn create_error(message: String) -> Value {
  unsafe {
    let message = create_string_utf8(&message);

    let mut result = MaybeUninit::<Value>::zeroed();
    napi_check!(napi_create_error, null_mut(), message, result.as_mut_ptr());

    result.assume_init()
  }
}

pub fn throw(error: Value) {
  unsafe {
    let status = napi_throw(Napi::env(), error);
    if status == Status::napi_ok {
      return
    }

    let location = format!("{} line {}", file!(), line!());
    let message = format!("Error throwing (status={:?})", status);
    nodejs_sys::napi_fatal_error(
      location.as_ptr() as *const raw::c_char,
      location.len(),
      message.as_ptr() as *const raw::c_char,
      message.len());
  }
}

// ========================================================================== //
// FUNCTIONS CALLBACK                                                         //
// ========================================================================== //

struct CallbackWrapper {
  callback: Box<dyn Fn(Value, Vec<Value>) -> NapiResult<Value> + 'static>
}

impl CallbackWrapper {
  fn call(&self, this: Value, args: Vec<Value>) -> NapiResult<Value> {
    let callback = &self.callback;
    callback(this, args)
  }
}

pub struct Callback {
  this: Value,
  args: Vec<Value>,
  wrapper: &'static CallbackWrapper,
}

impl Callback {
  pub fn call(self) -> NapiResult<Value> {
    self.wrapper.call(self.this, self.args)
  }
}

extern "C" fn callback_trampoline(env: napi_env, info: napi_callback_info) -> Value {
  let env = Napi::new(env);

  // Call up our initialization function with exports wrapped in a NapiObject
  // and unwrap the result into a simple "napi_value" (the pointer)
  let panic = panic::catch_unwind(|| {
    let callback = get_cb_info(info);
    callback.call()
  });

  // See if the initialization panicked
  let result = panic.unwrap_or_else(|error| {
    Err(NapiError::from(format!("PANIC: {:?}", error)))
  });

  // When we get here, we dealt with possible panic situations, now we have
  // a result, which (if OK) will hold the napi_value to return to node or
  // (if ERR) will contain a NapiError to throw before returning
  if let Err(error) = result {
    throw(error.into());
  }

  // All done...
  drop(env);
  return ptr::null_mut()
}

pub fn get_cb_info(info: CallbackInfo) -> Callback {
  unsafe {
    let mut argc = MaybeUninit::<usize>::zeroed();
    let mut this = MaybeUninit::<Value>::zeroed();
    let mut data = MaybeUninit::<*mut raw::c_void>::zeroed();

    // Figure out arguments count, "this" and our data (NapiCallbackWrapper)
    napi_check!(
      napi_get_cb_info,
      info,
      argc.as_mut_ptr(),
      ptr::null_mut(),
      this.as_mut_ptr(),
      data.as_mut_ptr()
    );

    // If we have arguments, extract them from our call info
    let args = match argc.assume_init() < 1 {
      true => vec![], // no args
      false => {
        let mut argv = vec![ptr::null_mut(); argc.assume_init()];
        napi_check!(
          napi_get_cb_info,
          info,
          argc.as_mut_ptr(),
          argv.as_mut_ptr(),
          ptr::null_mut(),
          ptr::null_mut()
        );
        argv
      }
    };

    // Build up our CallbacKData
    let pointer = data.assume_init() as *mut CallbackWrapper;
    let wrapper = &*{pointer};

    Callback { this: this.assume_init(), args, wrapper }
  }
}

/* ========================================================================== *
 * TYPES RELATED                                                              *
 * ========================================================================== */

pub fn type_of(value: Value) -> ValueType {
  unsafe {
    let mut result = MaybeUninit::<ValueType>::zeroed();
    napi_check!(napi_typeof, value, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn coerce_to_string(value: Value) -> String {
  unsafe {
    let mut result = MaybeUninit::<Value>::zeroed();
    napi_check!(napi_coerce_to_string, value, result.as_mut_ptr());
    get_value_string_utf8(result.assume_init())
  }
}

// ===== BIGINT ================================================================

pub fn create_bigint_words(value: i128) -> Value {
  let (sign, unsigned) = match value.is_negative() {
    true => (1, value.overflowing_neg().0),
    false => (0, value),
  };

  unsafe {
    let mut result = MaybeUninit::<Value>::zeroed();
    napi_check!(
      napi_create_bigint_words,
      sign,
      2,
      unsigned.to_le_bytes().as_ptr() as *mut u64,
      result.as_mut_ptr()
    );
    result.assume_init()
  }
}

pub fn get_value_bigint_words(value: Value) -> i128 {
  unsafe {
    let mut sign = MaybeUninit::<i32>::zeroed();
    let mut words = MaybeUninit::<usize>::new(2);
    let mut result = MaybeUninit::<i128>::zeroed();
    napi_check!(napi_get_value_bigint_words,
      value,
      sign.as_mut_ptr(),
      words.as_mut_ptr(),
      result.as_mut_ptr() as *mut u64
    );

    let sign = sign.assume_init();
    let words = words.assume_init();
    let result = result.assume_init();

    // Quick, no more than two words!
    if words > 2 {
      panic!("Unable to convert JavaScript \"bigint\" to Rust \"i128\" (requires {} words)", words)
    }

    // If the result (i128) was _negative_ but Node says it's positive then we
    // have an overflow on the top bit
    if (sign == 0) && (result < 0) {
      panic!("Unable to convert JavaScript \"bigint\" to Rust \"i128\" (requires 129 bits")
    }

    // Otherwise we're pretty much done!
    result
  }
}

// ===== BOOLEAN ===============================================================

pub fn get_boolean(value: bool) -> Value {
  unsafe {
    let mut result = MaybeUninit::<Value>::zeroed();
    napi_check!(napi_get_boolean, value, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn get_value_bool(value: Value) -> bool {
  unsafe {
    let mut result = MaybeUninit::<bool>::zeroed();
    napi_check!(napi_get_value_bool, value, result.as_mut_ptr());
    result.assume_init()
  }
}

// ===== FUNCTION ==============================================================

pub fn create_function<F>(name: &str, callback: F) -> Value
where
  F: Fn(Value, Vec<Value>) -> NapiResult<Value> + Sized + 'static
{
  let wrapper = CallbackWrapper { callback: Box::new(callback) };
  let boxed = Box::new(wrapper);
  let pointer = Box::into_raw(boxed);

  // todo: finalize!

  // Shove everything in our wrapper...
  unsafe {
    let mut result = MaybeUninit::<Value>::zeroed();
    napi_check!(
      napi_create_function,
      name.as_ptr() as *const raw::c_char,
      name.len(),
      Some(callback_trampoline),
      pointer as *mut raw::c_void,
      result.as_mut_ptr()
    );
    result.assume_init()
  }
}

pub fn call_function(this: Value, function: Value, args: Vec<Value>) -> NapiResult<Value> {
  unsafe {
    let mut result = MaybeUninit::<Value>::zeroed();

    napi_call_function(
      Napi::env(),
      this,
      function,
      args.len(),
      args.as_ptr(),
      result.as_mut_ptr(),
    );

    if ! is_exception_pending() {
      return Ok(result.assume_init())
    }

    let mut error = MaybeUninit::<Value>::zeroed();
    napi_get_and_clear_last_exception(Napi::env(), error.as_mut_ptr());
    Err(error.assume_init().into())
  }
}

// ===== NULL ==================================================================

pub fn get_null() -> Value {
  unsafe {
    let mut result = MaybeUninit::<Value>::zeroed();
    napi_check!(napi_get_null, result.as_mut_ptr());
    result.assume_init()
  }
}

// ===== NUMBER ================================================================

pub fn create_double(value: f64) -> Value {
  unsafe {
    let mut result = MaybeUninit::<Value>::zeroed();
    napi_check!(napi_create_double, value, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn get_value_double(value: Value) -> f64 {
  unsafe {
    let mut result = MaybeUninit::<f64>::zeroed();
    napi_check!(napi_get_value_double, value, result.as_mut_ptr());
    result.assume_init()
  }
}

// ===== OBJECT ================================================================

pub fn create_object() -> Value {
  unsafe {
    let mut result = MaybeUninit::<Value>::zeroed();
    napi_check!(napi_create_object, result.as_mut_ptr());
    result.assume_init()
  }
}

// ===== STRING ================================================================

pub fn create_string_utf8(string: &str) -> Value {
  unsafe {
    let mut result = MaybeUninit::<Value>::zeroed();
    napi_check!(napi_create_string_utf8,
      string.as_ptr() as *const raw::c_char,
      string.len(),
      result.as_mut_ptr()
    );
    result.assume_init()
  }
}

pub fn get_value_string_utf8(value: Value) -> String {
  unsafe {
    let mut size = MaybeUninit::<usize>::zeroed();

    // First, get the string *length* in bytes (it's safe, UTF8)
    napi_check!(napi_get_value_string_utf8, value, ptr::null_mut(), 0, size.as_mut_ptr());

    // Allocate a buffer of the correct size (plus 1 for null)
    let mut buffer = vec![0; size.assume_init() + 1];

    // Now properly get the string data
    napi_check!(
      napi_get_value_string_utf8,
      value,
      buffer.as_mut_ptr() as *mut raw::c_char,
      buffer.len(),
      size.as_mut_ptr()
    );

    // Slice up the buffer, removing the trailing null terminator
    String::from_utf8_unchecked(buffer[0..size.assume_init()].to_vec())
  }
}

// ===== SYMBOL ================================================================

pub fn create_symbol(value: Value) -> Value {
  unsafe {
    let mut result = MaybeUninit::<Value>::zeroed();
    napi_check!(napi_create_symbol, value, result.as_mut_ptr());
    result.assume_init()
  }
}

// this doesn't seem to esist in "nodejs_sys"
extern "C" {
  fn node_api_symbol_for(
    env: napi_env,
    descr: *const raw::c_char,
    length: usize,
    result: *mut Value,
  ) -> napi_status;
}

pub fn symbol_for(description: &str) -> Value {
  unsafe {
    let mut result = MaybeUninit::<Value>::zeroed();

    napi_check!(
      node_api_symbol_for,
      description.as_ptr() as *const raw::c_char,
      description.len(),
      result.as_mut_ptr()
    );

    result.assume_init()
  }
}

// ===== UNDEFINED =============================================================

pub fn get_undefined() -> Value {
  unsafe {
    let mut result = MaybeUninit::<Value>::zeroed();
    napi_check!(napi_get_undefined, result.as_mut_ptr());
    result.assume_init()
  }
}

/* ========================================================================== *
 * PROPERTIES                                                                 *
 * ========================================================================== */

pub fn set_named_property(object: Value, key: Value, value: Value) {
  unsafe {
    napi_check!(napi_set_property, object, key, value);
  }
}

pub fn get_named_property(object: Value, key: Value) -> Value {
  unsafe {
    let mut result = MaybeUninit::<Value>::zeroed();
    napi_get_property(Napi::env(), object, key, result.as_mut_ptr());
    result.assume_init()
  }
}

// ========================================================================== //
// REFERENCING                                                                //
// ========================================================================== //

pub fn create_reference(value: Value, initial: u32) -> Reference {
  unsafe {
    let mut result = MaybeUninit::<Reference>::zeroed();
    napi_check!(napi_create_reference, value, initial, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn reference_ref(reference: Reference) -> u32 {
  unsafe {
    let mut result = MaybeUninit::<u32>::zeroed();
    napi_reference_ref(Napi::env(), reference, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn reference_unref(reference: Reference) -> u32 {
  unsafe {
    let mut result = MaybeUninit::<u32>::zeroed();
    napi_reference_unref(Napi::env(), reference, result.as_mut_ptr());
    result.assume_init()
  }
}
