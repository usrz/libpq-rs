use super::*;

use nodejs_sys::*;
use std::mem::MaybeUninit;
use std::os::raw;
use std::ptr;

pub fn type_of(value: Handle) -> Type {
  unsafe {
    let mut result = MaybeUninit::<Type>::zeroed();
    napi_check!(napi_typeof, value, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn coerce_to_string(value: Handle) -> String {
  unsafe {
    let mut result = MaybeUninit::<Handle>::zeroed();
    napi_check!(napi_coerce_to_string, value, result.as_mut_ptr());
    get_value_string_utf8(result.assume_init())
  }
}

// ===== BIGINT ================================================================

pub fn create_bigint_words(value: i128) -> Handle {
  let (sign, unsigned) = match value.is_negative() {
    true => (1, value.overflowing_neg().0),
    false => (0, value),
  };

  unsafe {
    let mut result = MaybeUninit::<Handle>::zeroed();
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

pub fn get_value_bigint_words(value: Handle) -> i128 {
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

pub fn get_boolean(value: bool) -> Handle {
  unsafe {
    let mut result = MaybeUninit::<Handle>::zeroed();
    napi_check!(napi_get_boolean, value, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn get_value_bool(value: Handle) -> bool {
  unsafe {
    let mut result = MaybeUninit::<bool>::zeroed();
    napi_check!(napi_get_value_bool, value, result.as_mut_ptr());
    result.assume_init()
  }
}

// ===== NULL ==================================================================

pub fn get_null() -> Handle {
  unsafe {
    let mut result = MaybeUninit::<Handle>::zeroed();
    napi_check!(napi_get_null, result.as_mut_ptr());
    result.assume_init()
  }
}

// ===== NUMBER ================================================================

pub fn create_double(value: f64) -> Handle {
  unsafe {
    let mut result = MaybeUninit::<Handle>::zeroed();
    napi_check!(napi_create_double, value, result.as_mut_ptr());
    result.assume_init()
  }
}

pub fn get_value_double(value: Handle) -> f64 {
  unsafe {
    let mut result = MaybeUninit::<f64>::zeroed();
    napi_check!(napi_get_value_double, value, result.as_mut_ptr());
    result.assume_init()
  }
}

// ===== STRING ================================================================

pub fn create_string_utf8(string: &str) -> Handle {
  unsafe {
    let mut result = MaybeUninit::<Handle>::zeroed();
    napi_check!(napi_create_string_utf8,
      string.as_ptr() as *const raw::c_char,
      string.len(),
      result.as_mut_ptr()
    );
    result.assume_init()
  }
}

pub fn get_value_string_utf8(value: Handle) -> String {
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

pub fn create_symbol(value: Handle) -> Handle {
  unsafe {
    let mut result = MaybeUninit::<Handle>::zeroed();
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
    result: *mut Handle,
  ) -> napi_status;
}

pub fn symbol_for(description: &str) -> Handle {
  unsafe {
    let mut result = MaybeUninit::<Handle>::zeroed();

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

pub fn get_undefined() -> Handle {
  unsafe {
    let mut result = MaybeUninit::<Handle>::zeroed();
    napi_check!(napi_get_undefined, result.as_mut_ptr());
    result.assume_init()
  }
}
