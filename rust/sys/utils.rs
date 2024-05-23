use std::os::raw::c_char;

pub fn to_str(s: *const c_char) -> Result<&'static str, String> {
  let buffer = unsafe { std::ffi::CStr::from_ptr(s) };
  let result = buffer.to_str();
  match result {
    Err(_) => Err("Error decoding UTF-8 string".to_string()),
    Ok(result) => Ok(result),
  }
}

pub fn to_string(s: *const c_char) -> Result<String, String> {
  let str = to_str(s)?;
  Ok(str.to_string())
}

pub fn to_cstring(s: &str) -> std::ffi::CString {
  unsafe { std::ffi::CString::from_vec_unchecked(s.as_bytes().to_vec()) }
}

pub struct NullTerminatedArray {
  strings: Vec<std::ffi::CString>,
}

impl NullTerminatedArray {
  pub fn new<S: ToString>(vec: &[S]) -> Self {
    let strings = vec
    .iter()
    .map(|string| to_cstring(&string.to_string()))
    .collect::<Vec<_>>();
    Self{ strings }
  }

  pub fn from_raw(raw: *const *const c_char) -> Result<Self, String> {
    let mut strings = Vec::<std::ffi::CString>::new();

    for x in 0.. {
      unsafe {
        if (*raw.offset(x)).is_null() {
          break;
        } else {
          let ptr = *raw.offset(x);
          let cstr = std::ffi::CStr::from_ptr(ptr);
          let vec = cstr.to_bytes().to_vec();
          let cstring = std::ffi::CString::from_vec_unchecked(vec);
          strings.push(cstring);
        }
      }
    }

    Ok(Self{ strings })
  }

  pub unsafe fn as_vec(&self) -> Vec<*const c_char> {
    let mut pointers = self.strings
    .iter()
    .map(|cstring| cstring.as_ptr())
    .collect::<Vec<_>>();

    pointers.push(std::ptr::null());
    pointers
  }
}
