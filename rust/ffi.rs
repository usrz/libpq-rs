use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null;

/* ========================================================================== *
 * CONVERSION FUNCTIONS                                                       *
 * ========================================================================== */

/// Attempt to convert an external "* const c_char" (a standard null-terminated
/// _string_ in "C" parlance) into a proper Rust `String`, cloning the bytes.
///
pub fn to_string(s: *const c_char) -> Result<String, String> {
  let buffer = unsafe { CStr::from_ptr(s) };
  let result = buffer.to_str();
  match result {
    Err(_) => Err("Error decoding UTF-8 string".to_string()),
    Ok(result) => Ok(result.to_string()),
  }
}

/// Converts a borrowed `Rust` string into an `ffi:CString` (we can use the
/// unsafe `CString::as_ptr()` to pass it to standard "C" functions).
///
pub fn to_cstring(s: &str) -> CString {
  unsafe { CString::from_vec_unchecked(s.as_bytes().to_vec()) }
}

/* ========================================================================== *
 * NULL TERMINATED ARRAY                                                      *
 * ========================================================================== */

/// A wrapper for an array of "C" strings with a null pointer at the end.
///
pub struct NullTerminatedArray {
  strings: Vec<String>,
}

impl From<Vec<String>> for NullTerminatedArray {
  /// Create a new instance from a vector of `ToString`s.
  ///
  fn from(strings: Vec<String>) -> Self {
    Self{ strings }
  }
}

impl From<Vec<&str>> for NullTerminatedArray {
  /// Create a new instance from a vector of `ToString`s.
  ///
  fn from(strings: Vec<&str>) -> Self {
    let strings = strings
      .iter()
      .map(|str| str.to_string())
      .collect();
    Self{ strings }
  }
}

impl NullTerminatedArray {
  /// Creaet a new instance from a null-terminated array of "C" strings
  ///
  pub unsafe fn from_raw(raw: *const *const c_char) -> Result<Self, String> {
    let mut strings = Vec::<String>::new();

    for x in 0.. {
      if (*raw.offset(x)).is_null() {
        break;
      } else {
        let ptr = *raw.offset(x);
        let string = to_string(ptr)?;
        strings.push(string);
      }
    }

    Ok(Self{ strings })
  }

  /// Return this as a null-terminated array of "C" string
  ///
  pub fn as_vec(&self) -> Vec<*const c_char> {
    let mut pointers = self.strings
      .iter()
      .map(|string| to_cstring(string).as_ptr())
      .collect::<Vec<_>>();

    pointers.push(null());
    pointers
  }
}
