use crate::env::Napi;
use crate::errors::*;

use nodejs_sys::*;
use std::mem::MaybeUninit;
use std::ptr::null_mut;
use std::os::raw;

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
    napi_check!(napi_get_value_string_utf8, value, null_mut(), 0, size.as_mut_ptr());

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
