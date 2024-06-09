use super::*;

use nodejs_sys::*;
use std::mem::MaybeUninit;
use std::os::raw;
use std::ptr;

pub fn type_of(env: Env, handle: Handle) -> TypeOf {
  unsafe {
    let mut result = MaybeUninit::<TypeOf>::zeroed();
    napi_check!(napi_typeof, env, handle, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn expect_type_of(env: Env, handle: Handle, expected: TypeOf) {
  unsafe {
    let mut result = MaybeUninit::<TypeOf>::zeroed();
    napi_check!(napi_typeof, env, handle, result.as_mut_ptr());

    let actual = result.assume_init();
    if actual != expected {
      panic!("Expected type {:?}, actual {:?}", expected, actual)
    }
  }
}

pub fn coerce_to_string(env: Env, handle: Handle) -> String {
  unsafe {
    let mut result = MaybeUninit::<Handle>::zeroed();
    napi_check!(napi_coerce_to_string, env, handle, result.as_mut_ptr());
    get_value_string_utf8(env, result.assume_init())
  }
}

// ===== BIGINT ================================================================

pub fn create_bigint_words(env: Env, value: i128) -> Handle {
  let (sign, unsigned) = match value.is_negative() {
    true => (1, value.overflowing_neg().0),
    false => (0, value),
  };

  unsafe {
    let mut result = MaybeUninit::<Handle>::zeroed();
    napi_check!(
      napi_create_bigint_words,
      env,
      sign,
      2,
      unsigned.to_le_bytes().as_ptr() as *mut u64,
      result.as_mut_ptr()
    );
    result.assume_init()
  }
}

pub fn get_value_bigint_words(env: Env, handle: Handle) -> i128 {
  unsafe {
    let mut sign = MaybeUninit::<i32>::zeroed();
    let mut words = MaybeUninit::<usize>::new(2);
    let mut result = MaybeUninit::<i128>::zeroed();
    napi_check!(napi_get_value_bigint_words,
      env,
      handle,
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

pub fn get_boolean(env: Env, value: bool) -> Handle {
  unsafe {
    let mut result = MaybeUninit::<Handle>::zeroed();
    napi_check!(napi_get_boolean, env, value, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn get_value_bool(env: Env, handle: Handle) -> bool {
  unsafe {
    let mut result = MaybeUninit::<bool>::zeroed();
    napi_check!(napi_get_value_bool, env, handle, result.as_mut_ptr());
    result.assume_init()
  }
}

// ===== NULL ==================================================================

pub fn get_null(env: Env, ) -> Handle {
  unsafe {
    let mut result = MaybeUninit::<Handle>::zeroed();
    napi_check!(napi_get_null, env, result.as_mut_ptr());
    result.assume_init()
  }
}

// ===== NUMBER ================================================================

pub fn create_double(env: Env, value: f64) -> Handle {
  unsafe {
    let mut result = MaybeUninit::<Handle>::zeroed();
    napi_check!(napi_create_double, env, value, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn get_value_double(env: Env, handle: Handle) -> f64 {
  unsafe {
    let mut result = MaybeUninit::<f64>::zeroed();
    napi_check!(napi_get_value_double, env, handle, result.as_mut_ptr());
    result.assume_init()
  }
}

// ===== STRING ================================================================

pub fn create_string_utf8(env: Env, value: &str) -> Handle {
  unsafe {
    let mut result = MaybeUninit::<Handle>::zeroed();
    napi_check!(
      napi_create_string_utf8,
      env,
      value.as_ptr() as *const raw::c_char,
      value.len(),
      result.as_mut_ptr()
    );
    result.assume_init()
  }
}

pub fn get_value_string_utf8(env: Env, handle: Handle) -> String {
  unsafe {
    let mut size = MaybeUninit::<usize>::zeroed();

    // First, get the string *length* in bytes (it's safe, UTF8)
    napi_check!(
      napi_get_value_string_utf8,
      env,
      handle,
      ptr::null_mut(),
      0,
      size.as_mut_ptr()
    );

    // Allocate a buffer of the correct size (plus 1 for null)
    let mut buffer = vec![0; size.assume_init() + 1];

    // Now properly get the string data
    napi_check!(
      napi_get_value_string_utf8,
      env,
      handle,
      buffer.as_mut_ptr() as *mut raw::c_char,
      buffer.len(),
      size.as_mut_ptr()
    );

    // Slice up the buffer, removing the trailing null terminator
    String::from_utf8_unchecked(buffer[0..size.assume_init()].to_vec())
  }
}

// ===== SYMBOL ================================================================

pub fn create_symbol(env: Env, handle: Handle) -> Handle {
  unsafe {
    let mut result = MaybeUninit::<Handle>::zeroed();
    napi_check!(
      napi_create_symbol,
      env,
      handle,
      result.as_mut_ptr()
    );
    result.assume_init()
  }
}

// this doesn't seem to esist in "nodejs_sys"
extern "C" {
  fn node_api_symbol_for(
    env: napi_env,
    descr: *const raw::c_char,
    length: usize,
    result: *mut Handle,
  ) -> napi_status;
}

pub fn symbol_for(env: Env, description: &str) -> Handle {
  unsafe {
    let mut result = MaybeUninit::<Handle>::zeroed();

    napi_check!(
      node_api_symbol_for,
      env,
      description.as_ptr() as *const raw::c_char,
      description.len(),
      result.as_mut_ptr()
    );

    result.assume_init()
  }
}

// ===== UNDEFINED =============================================================

pub fn get_undefined(env: Env, ) -> Handle {
  unsafe {
    let mut result = MaybeUninit::<Handle>::zeroed();
    napi_check!(napi_get_undefined, env, result.as_mut_ptr());
    result.assume_init()
  }
}
