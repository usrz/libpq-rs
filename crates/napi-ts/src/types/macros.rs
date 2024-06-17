macro_rules! napi_type {
  (
    $(#[$outer:meta])*
    $type:ident // The final type, e.g. NapiObject
    $(<$($params:ident),+>)?, // Any generic parameters
    $value:ident, // The NapiValue type to associate with this
    $def:tt // The block defining the structure
  ) => {
    // impl NapiType for NapiBoolean
    impl <'a, $($($params: 'static,)?)?> NapiType<'a> for $type<'a, $($($params,)?)?> {
      // Marker type
    }

    // impl NapiTypeInternal for NapiBoolean
    impl <'a, $($($params: 'static,)?)?> NapiTypeInternal<'a> for $type<'a, $($($params,)?)?> $def

    // impl NapiTypeIdInternal for NapiBoolean
    impl $(<$($params: 'static,)?>)? NapiTypeWithTypeOf for $type<'_, $($($params,)?)?> {
      const TYPE_OF: Option<NapiTypeOf> = Some(NapiTypeOf::$value);
    }

    // impl Debug for NapiBoolean
    impl $(<$($params: 'static,)?>)? std::fmt::Debug for $type<'_, $($($params,)?)?> {
      fn fmt(&self, fm: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          fm.debug_tuple(stringify!($type))
            .field(&self.napi_handle())
            .finish()
      }
    }
  }
}

pub (super) use napi_type;
