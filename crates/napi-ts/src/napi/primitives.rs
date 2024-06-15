use crate::napi::*;

impl Env {

  pub fn type_of(&self, handle: &Handle) -> NapiTypeOf {
    unsafe {
      let mut result = MaybeUninit::<napi_valuetype>::zeroed();
      env_check!(
        napi_typeof,
        self,
        handle.value,
        result.as_mut_ptr()
      );

      let value = result.assume_init().into();
      match value  {
        napi_valuetype::napi_undefined => NapiTypeOf::Undefined,
        napi_valuetype::napi_null => NapiTypeOf::Null,
        napi_valuetype::napi_boolean => NapiTypeOf::Boolean,
        napi_valuetype::napi_number => NapiTypeOf::Number,
        napi_valuetype::napi_string => NapiTypeOf::String,
        napi_valuetype::napi_symbol => NapiTypeOf::Symbol,
        napi_valuetype::napi_object => NapiTypeOf::Object,
        napi_valuetype::napi_function => NapiTypeOf::Function,
        napi_valuetype::napi_external => NapiTypeOf::External,
        napi_valuetype::napi_bigint => NapiTypeOf::Bigint,
        #[allow(unreachable_patterns)] // this should *really* never happen...
        _ => panic!("Unsupported JavaScript type \"{:?}\"", value)
      }

    }
  }

  pub fn coerce_to_string(&self, handle: &Handle) -> String {
    unsafe {
      let mut result: MaybeUninit<napi_value> = MaybeUninit::zeroed();
      env_check!(
        napi_coerce_to_string,
        self,
        handle.value,
        result.as_mut_ptr()
      );

      let value = self.handle(result.assume_init());

      self.get_value_string_utf8(&value)
    }
  }

  // ===== BIGINT ================================================================

  pub fn create_bigint_words(&self, value: i128) -> Handle {
    let (sign, unsigned) = match value.is_negative() {
      true => (1, value.overflowing_neg().0),
      false => (0, value),
    };

    unsafe {
      let mut result: MaybeUninit<napi_value> = MaybeUninit::zeroed();
      env_check!(
        napi_create_bigint_words,
        self,
        sign,
        2,
        unsigned.to_le_bytes().as_ptr() as *mut u64,
        result.as_mut_ptr()
      );

      self.handle(result.assume_init())
    }
  }

  pub fn get_value_bigint_words(&self, handle: &Handle) -> i128 {
    unsafe {
      let mut sign = MaybeUninit::<i32>::zeroed();
      let mut words = MaybeUninit::<usize>::new(2);
      let mut result = MaybeUninit::<i128>::zeroed();
      env_check!(
        napi_get_value_bigint_words,
        self,
        handle.value,
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

  pub fn get_boolean(&self, value: bool) -> Handle {
    unsafe {
      let mut result: MaybeUninit<napi_value> = MaybeUninit::zeroed();
      env_check!(
        napi_get_boolean,
        self,
        value,
        result.as_mut_ptr()
      );

      self.handle(result.assume_init())
    }
  }

  pub fn get_value_bool(&self, handle: &Handle) -> bool {
    unsafe {
      let mut result = MaybeUninit::<bool>::zeroed();
      env_check!(
        napi_get_value_bool,
        self,
        handle.value,
        result.as_mut_ptr()
      );

      result.assume_init()
    }
  }

  // ===== NULL ==================================================================

  pub fn get_null(&self) -> Handle {
    unsafe {
      let mut result: MaybeUninit<napi_value> = MaybeUninit::zeroed();
      env_check!(
        napi_get_null,
        self,
        result.as_mut_ptr()
      );

      self.handle(result.assume_init())
    }
  }

  // ===== NUMBER ================================================================

  pub fn create_double(&self, value: f64) -> Handle {
    unsafe {
      let mut result: MaybeUninit<napi_value> = MaybeUninit::zeroed();
      env_check!(
        napi_create_double,
        self,
        value,
        result.as_mut_ptr()
      );

      self.handle(result.assume_init())
    }
  }

  pub fn get_value_double(&self, handle: &Handle) -> f64 {
    unsafe {
      let mut result = MaybeUninit::<f64>::zeroed();
      env_check!(
        napi_get_value_double,
        self,
        handle.value,
        result.as_mut_ptr()
      );

      result.assume_init()
    }
  }

  // ===== STRING ================================================================

  pub fn create_string_utf8(&self, value: &str) -> Handle {
    unsafe {
      let mut result: MaybeUninit<napi_value> = MaybeUninit::zeroed();
      env_check!(
        napi_create_string_utf8,
        self,
        value.as_ptr() as *const raw::c_char,
        value.len(),
        result.as_mut_ptr()
      );

      self.handle(result.assume_init())
    }
  }

  pub fn get_value_string_utf8(&self, handle: &Handle) -> String {
    unsafe {
      let mut size = MaybeUninit::<usize>::zeroed();

      // First, get the string *length* in bytes (it's safe, UTF8)
      env_check!(
        napi_get_value_string_utf8,
        self,
        handle.value,
        ptr::null_mut(),
        0,
        size.as_mut_ptr()
      );

      // Allocate a buffer of the correct size (plus 1 for null)
      let mut buffer = vec![0; size.assume_init() + 1];

      // Now properly get the string data
      env_check!(
        napi_get_value_string_utf8,
        self,
        handle.value,
        buffer.as_mut_ptr() as *mut raw::c_char,
        buffer.len(),
        size.as_mut_ptr()
      );

      // Slice up the buffer, removing the trailing null terminator
      String::from_utf8_unchecked(buffer[0..size.assume_init()].to_vec())
    }
  }

  // ===== SYMBOL ================================================================

  pub fn create_symbol(&self, description: Option<&str>) -> Handle {
    let handle = match description {
      Some(description) => self.create_string_utf8(description),
      None => self.get_undefined(),
    };

    unsafe {
      let mut result: MaybeUninit<napi_value> = MaybeUninit::zeroed();
      env_check!(
        napi_create_symbol,
        self,
        handle.value,
        result.as_mut_ptr()
      );

      self.handle(result.assume_init())
    }
  }

  pub fn symbol_for(&self, description: &str) -> Handle {
    unsafe {
      let mut result: MaybeUninit<napi_value> = MaybeUninit::zeroed();

      env_check!(
        node_api_symbol_for,
        self,
        description.as_ptr() as *const raw::c_char,
        description.len(),
        result.as_mut_ptr()
      );

      self.handle(result.assume_init())
    }
  }

  // ===== UNDEFINED =============================================================

  pub fn get_undefined(&self) -> Handle {
    unsafe {
      let mut result: MaybeUninit<napi_value> = MaybeUninit::zeroed();
      env_check!(napi_get_undefined, self, result.as_mut_ptr());
      self.handle(result.assume_init())
    }
  }
}

impl Handle {
  pub fn type_of(&self) -> NapiTypeOf {
    self.env.type_of(self)
  }

  pub fn coerce_to_string(&self) -> String {
    self.env.coerce_to_string(self)
  }

  pub fn get_value_bigint_words(&self) -> i128 {
    self.env.get_value_bigint_words(self)
  }

  pub fn get_value_bool(&self) -> bool {
    self.env.get_value_bool(self)
  }

  pub fn get_value_double(&self) -> f64 {
    self.env.get_value_double(self)
  }

  pub fn get_value_string_utf8(&self) -> String {
    self.env.get_value_string_utf8(self)
  }

  pub fn get_undefined(&self) -> Handle {
    self.env.get_undefined()
  }

}
