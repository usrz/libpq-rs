use crate::napi::*;

impl Env {
  /// Checks whether an exception is pending in NodeJS
  ///
  /// See [`napi_is_exception_pending`](https://nodejs.org/api/n-api.html#napi_is_exception_pending)
  ///
  pub fn is_exception_pending(&self) -> bool {
    unsafe {
      let mut result = MaybeUninit::<bool>::zeroed();
      env_check!(
        napi_is_exception_pending,
        self,
        result.as_mut_ptr()
      );
      result.assume_init()
    }
  }

  pub fn is_error(&self, handle: &Handle) -> bool {
    unsafe {
      let mut result = MaybeUninit::<bool>::zeroed();
      env_check!(
        napi_is_error,
        self,
        handle.value,
        result.as_mut_ptr()
      );
      result.assume_init()
    }
  }

  /// Create a JavaScript `Error` with the provided text.
  ///
  /// See [`napi_create_error`](https://nodejs.org/api/n-api.html#napi_create_error)
  ///
  pub fn create_error(&self, message: &str) -> Handle {
    unsafe {
      let message = self.create_string_utf8(message);

      let mut result: MaybeUninit<napi_value> = MaybeUninit::zeroed();
      env_check!(
        napi_create_error,
        self,
        ptr::null_mut(),
        message.value,
        result.as_mut_ptr()
      );

      self.handle(result.assume_init())
    }
  }

  /// Create a JavaScript `TypeError` with the provided text.
  ///
  /// See [`napi_create_type_error`](https://nodejs.org/api/n-api.html#napi_create_type_error)
  ///
  pub fn create_type_error(&self, message: &str) -> Handle {
    unsafe {
      let message = self.create_string_utf8(message);

      let mut result: MaybeUninit<napi_value> = MaybeUninit::zeroed();
      env_check!(
        napi_create_type_error,
        self,
        ptr::null_mut(),
        message.value,
        result.as_mut_ptr()
      );

      self.handle(result.assume_init())
    }
  }

  /// Create a JavaScript `RangeError` with the provided text.
  ///
  /// See [`napi_create_range_error`](https://nodejs.org/api/n-api.html#napi_create_range_error)
  ///
  pub fn create_range_error(&self, message: &str) -> Handle {
    unsafe {
      let message = self.create_string_utf8(message);

      let mut result: MaybeUninit<napi_value> = MaybeUninit::zeroed();
      env_check!(
        napi_create_range_error,
        self,
        ptr::null_mut(),
        message.value,
        result.as_mut_ptr()
      );

      self.handle(result.assume_init())
    }
  }

  /// Throw the JavaScript value provided (**don't use**).
  ///
  /// Normal code should always rely on returning the correct [`NapiResult`]
  /// to the caller (whether `Ok` or `Err`) and should never throw exceptions
  /// directly.
  ///
  /// After calling this function, very little can be done in the current
  /// environment until the exception is caught. Use with care.
  ///
  /// If an exception can not be thrown, the process will terminate calling
  /// [`napi_fatal_error`](https://nodejs.org/api/n-api.html#napi_fatal_error).
  ///
  /// See [`napi_throw`](https://nodejs.org/api/n-api.html#napi_throw)
  ///
  pub fn throw(&self, handle: &Handle) -> Handle {
    unsafe {
      let undefined = self.get_undefined(); // get this *first*
      let status = napi_throw(self.0, handle.value);
      if status == napi_status::napi_ok {
        return undefined
      }

      let location = format!("{} line {}", file!(), line!());
      let message = format!("Error throwing (status={:?})", status);
      napi_fatal_error(
        location.as_ptr() as *const raw::c_char,
        location.len(),
        message.as_ptr() as *const raw::c_char,
        message.len());
    }
  }
}

impl Handle {
  pub fn is_error(&self) -> bool {
    self.env.is_error(self)
  }

  pub fn throw(&self) -> Handle {
    self.env.throw(self)
  }
}
