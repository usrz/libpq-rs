macro_rules! napi_type {
  (
    $type:ident // The final type, e.g. NapiObject
    $(<$($params:ident),+>)?, // Any generic parameters
    $value:ident, // The NapiValue type to associate with this
    $def:tt // The block defining the structure
  ) => {
    pub struct $type$(<$($params: 'static,)?>)? $def

    // impl NapiType for NapiBoolean
    impl $(<$($params: 'static,)?>)? NapiType for $type$(<$($params,)?>)? {
      #[inline]
      fn into_napi_value(self) -> NapiValue {
        NapiValue::$value(self.napi_handle())
      }

      #[inline]
      fn try_from_napi_value(value: &NapiValue) -> Result<Self, NapiErr> {
        match value {
          NapiValue::$value(handle) => Ok($type::from_handle(*handle)),
          _ => Err(format!("Unable to downcast {} into {}", value, stringify!($type)).into())
        }
      }
    }

    // impl Into<NapiValue> for NapiBoolean
    impl $(<$($params: 'static,)?>)? Into<NapiValue> for $type$(<$($params,)?>)? {
      #[inline]
      fn into(self) -> NapiValue {
        NapiValue::$value(self.napi_handle())
      }
    }

    // impl Debug for NapiBoolean
    impl $(<$($params: 'static,)?>)? std::fmt::Debug for $type$(<$($params,)?>)? {
      fn fmt(&self, fm: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          fm.debug_tuple(stringify!($type))
            .field(&self.napi_handle())
            .finish()
      }
    }
  }
}

pub (super) use napi_type;
