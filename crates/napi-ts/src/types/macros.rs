macro_rules! napi_type {
  (
    $type:ident // The final type, e.g. NapiObject
    $(<$($params:ident),+>)?, // Any generic parameters
    $value:ident // The NapiValue type to associate with this
  ) => {
    // impl NapiType for NapiBoolean
    impl $(<$($params: 'static,)?>)? NapiType for $type$(<$($params,)?>)? {
      // marker
    }

    // impl Into<NapiValue> for NapiBoolean
    impl $(<$($params: 'static,)?>)? Into<NapiValue> for $type$(<$($params,)?>)? {
      fn into(self) -> NapiValue {
        NapiValue::$value(self.napi_handle())
      }
    }

    // impl Debug for NapiBoolean
    impl $(<$($params: 'static,)?>)? Debug for $type$(<$($params,)?>)? {
      fn fmt(&self, fm: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          fm.debug_tuple(stringify!($type))
            .field(&self.napi_handle())
            .finish()
      }
    }
  }
}

macro_rules! napi_value {
  (
    $type:ident // The final type, e.g. NapiObject
    $(<$($params:ident),+>)?, // Any generic parameters
    $value:ident // The NapiValue type to associate with this
  ) => {
    napi_type!($type$(<$($params,)?>)?, $value);

    // impl TryFrom<NapiValue> for NapiBoolean
    impl TryFrom<NapiValue> for $type$(<$($params,)?>)? {
      type Error = NapiErr;

      fn try_from(value: NapiValue) -> Result<Self, Self::Error> {
        match value {
          NapiValue::$value(handle) => Ok($type::from_handle(handle)),
          _ => Err(format!("Unable to downcast {} into {}", value, stringify!(NapiNull)).into())
        }
      }
    }
  }
}

pub (super) use napi_type;
pub (super) use napi_value;
