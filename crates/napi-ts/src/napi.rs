// pub use nodejs_sys;

use crate::env::Napi;
use crate::errors::*;

use nodejs_sys::*;
use std::mem::MaybeUninit;
use std::panic;
use std::ptr;
use std::os::raw;

pub type CallbackInfo = nodejs_sys::napi_callback_info;
pub type Env = nodejs_sys::napi_env;
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

// TODO: convert this to throw a proper "Error" alongside NapiResult/NapiError
pub fn throw_error(message: String, code: Option<String>) {
  let mut message = message.to_owned();
  message.push('\0'); // make sure we're null terminated!!!

  let code = match code {
    None => "\0".to_string(),
    Some(code) => {
      let mut code = code.to_owned();
      code.push('\0');
      code
    },
  };

  unsafe {
    let status = napi_throw_error(
      Napi::env(),
      code.as_ptr() as *const raw::c_char,
      message.as_ptr() as *const raw::c_char
    );

    if status == Status::napi_ok {
      return
    }

    let location = format!("{} line {}", file!(), line!());
    let message = format!("Error throwing \"{}\" (status={:?})", message, status);
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

extern "C" fn callback_trampoline(env: napi_env, info: napi_callback_info) -> napi_value {
  let env = unsafe { Napi::new(env) };

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
    throw_error(error.to_string(), None);
  }

  // All done...
  drop(env);
  return ptr::null_mut()
}

pub fn get_cb_info(info: CallbackInfo) -> Callback {
  unsafe {
    let mut argc = MaybeUninit::<usize>::zeroed();
    let mut this = MaybeUninit::<napi_value>::zeroed();
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

pub fn type_of(value: napi_value) -> ValueType {
  unsafe {
    let mut result = MaybeUninit::<ValueType>::zeroed();
    napi_check!(napi_typeof, value, result.as_mut_ptr());
    result.assume_init()
  }
}

// ===== BIGINT ================================================================

pub fn create_bigint_int64(value: i64) -> Value {
  unsafe {
    let mut result = MaybeUninit::<napi_value>::zeroed();
    napi_check!(napi_create_bigint_int64, value, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn create_bigint_uint64(value: u64) -> Value {
  unsafe {
    let mut result = MaybeUninit::<napi_value>::zeroed();
    napi_check!(napi_create_bigint_uint64, value, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn create_bigint_words_u128(value: u128) -> Value {
  let bytes = value.to_le_bytes();
  let words = bytes
    .chunks_exact(8)
    .map(|chunk| u64::from_ne_bytes(chunk.try_into().unwrap()))
    .collect::<Vec<_>>();

  unsafe {
    let mut result = MaybeUninit::<napi_value>::zeroed();
    napi_check!(napi_create_bigint_words, 0, words.len(), words.as_ptr(), result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn create_bigint_words_i128(value: i128) -> Value {
  let (sign, unsigned) = match value.is_negative() {
    true => (1, value.overflowing_neg().0),
    false => (0, value),
  };

  unsafe {
    let mut result = MaybeUninit::<napi_value>::zeroed();
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

// -----------------------------------------------------------------------------

pub fn get_value_bigint_int64(value: Value) -> NapiResult<i64> {
  unsafe {
    let mut result = MaybeUninit::<i64>::zeroed();
    let mut lossless = MaybeUninit::<bool>::new(true);
    napi_check!(napi_get_value_bigint_int64, value, result.as_mut_ptr(), lossless.as_mut_ptr());
    match lossless.assume_init() {
      true => Ok(result.assume_init()),
      false => Err(NapiError::from("Unable to convert JavaScript \"bigint\" to Rust \"i64\""))
    }
  }
}

pub fn get_value_bigint_uint64(value: Value) -> NapiResult<u64> {
  unsafe {
    let mut result = MaybeUninit::<u64>::zeroed();
    let mut lossless = MaybeUninit::<bool>::new(true);
    napi_check!(napi_get_value_bigint_uint64, value, result.as_mut_ptr(), lossless.as_mut_ptr());
    match lossless.assume_init() {
      true => Ok(result.assume_init()),
      false => Err(NapiError::from("Unable to convert JavaScript \"bigint\" to Rust \"u64\""))
    }
  }
}

pub fn get_value_bigint_words_i128(value: Value) -> NapiResult<i128> {
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
      return Err("Unable to convert JavaScript \"bigint\" to Rust \"i128\" a".into())
    }

    // If the result (i128) was _negative_ but Node says it's positive then we
    // have an overflow on the top bit
    if (sign == 0) && (result < 0) {
      return Err("Unable to convert JavaScript \"bigint\" to Rust \"i128\" a".into())
    }

    // Otherwise we're pretty much done!
    Ok(result)
  }
}

pub fn get_value_bigint_words_u128(value: Value) -> NapiResult<u128> {
  unsafe {
    let mut sign = MaybeUninit::<i32>::zeroed();
    let mut words = MaybeUninit::<usize>::new(2);
    let mut result = MaybeUninit::<[u8; 16]>::zeroed();
    napi_check!(napi_get_value_bigint_words,
      value,
      sign.as_mut_ptr(),
      words.as_mut_ptr(),
      result.as_mut_ptr() as *mut u64
    );

    // Initialized values
    let sign = sign.assume_init();
    let words = words.assume_init();
    let result = result.assume_init();

    // Check the sign (must be 0 or positive) and the number of words (must be 2 or less)
    if (sign != 0) || (words > 2) {
      return Err("Unable to convert JavaScript \"bigint\" to Rust \"u128\"".into())
    }

    // Convert LE bytes from node straight
    Ok(u128::from_le_bytes(result))
  }
}

// ===== BOOLEAN ===============================================================

pub fn get_boolean(value: bool) -> Value {
  unsafe {
    let mut result = MaybeUninit::<napi_value>::zeroed();
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

  // Shove everything in our wrapper...
  unsafe {
    let mut result = MaybeUninit::<napi_value>::zeroed();
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

// ===== NULL ==================================================================

pub fn get_null() -> Value {
  unsafe {
    let mut result = MaybeUninit::<napi_value>::zeroed();
    napi_check!(napi_get_null, result.as_mut_ptr());
    result.assume_init()
  }
}

// ===== NUMBER ================================================================

pub fn create_int32(value: i32) -> Value {
  unsafe {
    let mut result = MaybeUninit::<napi_value>::zeroed();
    napi_check!(napi_create_int32, value, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn create_uint32(value: u32) -> Value {
  unsafe {
    let mut result = MaybeUninit::<napi_value>::zeroed();
    napi_check!(napi_create_uint32, value, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn create_int64(value: i64) -> Value {
  unsafe {
    let mut result = MaybeUninit::<napi_value>::zeroed();
    napi_check!(napi_create_int64, value, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn create_double(value: f64) -> Value {
  unsafe {
    let mut result = MaybeUninit::<napi_value>::zeroed();
    napi_check!(napi_create_double, value, result.as_mut_ptr());
    result.assume_init()
  }
}

// -----------------------------------------------------------------------------

pub fn get_value_int32(value: Value) -> i32 {
  unsafe {
    let mut result = MaybeUninit::<i32>::zeroed();
    napi_check!(napi_get_value_int32, value, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn get_value_uint32(value: Value) -> u32 {
  unsafe {
    let mut result = MaybeUninit::<u32>::zeroed();
    napi_check!(napi_get_value_uint32, value, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn get_value_int64(value: Value) -> i64 {
  unsafe {
    let mut result = MaybeUninit::<i64>::zeroed();
    napi_check!(napi_get_value_int64, value, result.as_mut_ptr());
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
    let mut result = MaybeUninit::<napi_value>::zeroed();
    napi_check!(napi_create_object, result.as_mut_ptr());
    result.assume_init()
  }
}

// ===== STRING ================================================================

pub fn create_string_utf8(string: &str) -> Value {
  unsafe {
    let mut result = MaybeUninit::<napi_value>::zeroed();
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
    let mut result = MaybeUninit::<napi_value>::zeroed();
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
    result: *mut napi_value,
  ) -> napi_status;
}

pub fn symbol_for(description: &str) -> Value {
  unsafe {
    let mut result = MaybeUninit::<napi_value>::zeroed();

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
    let mut result = MaybeUninit::<napi_value>::zeroed();
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
    let mut result = MaybeUninit::<napi_value>::zeroed();
    napi_get_property(Napi::env(), object, key, result.as_mut_ptr());
    result.assume_init()
  }
}
