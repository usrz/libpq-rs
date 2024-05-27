//! Utilities to deal with the "C" types used by LibPQ.

use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null;

/* ========================================================================== *
 * CONVERSION FUNCTIONS                                                       *
 * ========================================================================== */

/// Convert a standard null-terminated _string_ (in "C" parlance) into a Rust
/// [`String`], ignoring any non-UTF8 sequences.
///
pub fn to_string_lossy(s: *const c_char) -> String {
  unsafe { CStr::from_ptr(s).to_string_lossy().to_string() }
}

/// Attempt to convert a standard null-terminated _string_ (in "C" parlance)
/// into a proper Rust [`String`], cloning the bytes.
///
pub fn to_string(s: *const c_char) -> Result<String, String> {
  let buffer = unsafe { CStr::from_ptr(s) };
  let result = buffer.to_str();
  match result {
    Err(_) => Err("Error decoding UTF-8 string".to_string()),
    Ok(result) => Ok(result.to_string()),
  }
}

/// Converts a borrowed [`str`]_ing_ into a [`CString`].
///
pub fn to_cstring(s: &str) -> CString {
  unsafe { CString::from_vec_unchecked(s.as_bytes().to_vec()) }
}

/* ========================================================================== *
 * NULL TERMINATED ARRAY                                                      *
 * ========================================================================== */

/// A wrapper for an array of "C" strings with a null pointer at the end.
///
#[derive(Debug)]
pub struct NullTerminatedArray {
  strings: Vec<CString>,
}

impl From<Vec<String>> for NullTerminatedArray {
  /// Create a [`NullTerminatedArray`] from a vector of [`String`]s.
  ///
  fn from(value: Vec<String>) -> Self {
    let strings = value
      .iter()
      .map(|string| to_cstring(string))
      .collect();

    Self{ strings }
  }
}

impl From<Vec<&str>> for NullTerminatedArray {
  /// Create a [`NullTerminatedArray`] from a vector of borrowed [`str`]_ings_.
  ///
  fn from(value: Vec<&str>) -> Self {
    let strings = value
      .iter()
      .map(|str| to_cstring(str))
      .collect();
    Self{ strings }
  }
}

impl NullTerminatedArray {
  /// Create a [`NullTerminatedArray`] from a null-terminated array of "C"
  /// strings.
  ///
  pub unsafe fn from_raw(raw: *const *const c_char) -> Result<Self, String> {
    let mut strings = Vec::<CString>::new();

    for x in 0.. {
      if (*raw.offset(x)).is_null() {
        break;
      } else {
        let ptr = *raw.offset(x);
        let cstr = CStr::from_ptr(ptr);
        let vec = cstr.to_bytes().to_vec();
        let cstring = CString::from_vec_unchecked(vec);
        strings.push(cstring);
      }
    }

    Ok(Self{ strings })
  }

  /// Return this as a null-terminated array of "C" string.
  ///
  pub fn as_vec(&self) -> Vec<*const c_char> {
    let mut pointers = self.strings
      .iter()
      .map(|cstring| cstring.as_ptr())
      .collect::<Vec<_>>();

    pointers.push(null());
    pointers
  }
}
