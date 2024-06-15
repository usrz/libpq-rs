macro_rules! napi_type {
  (
    $(#[$outer:meta])*
    $type:ident // The final type, e.g. NapiObject
    $(<$($params:ident),+>)?, // Any generic parameters
    $value:ident, // The NapiValue type to associate with this
    $def:tt // The block defining the structure
  ) => {
    // impl NapiType for NapiBoolean
    impl $(<$($params: 'static,)?>)? NapiType for $type$(<$($params,)?>)? {
      // Marker type
    }

    // impl NapiTypeInternal for NapiBoolean
    impl $(<$($params: 'static,)?>)? NapiTypeInternal for $type$(<$($params,)?>)? $def

    // impl NapiTypeIdInternal for NapiBoolean
    impl $(<$($params: 'static,)?>)? NapiTypeIdInternal for $type$(<$($params,)?>)? {
      fn has_type_of(type_of: crate::TypeOf) -> bool {
        crate::TypeOf::$value == type_of
      }

      fn type_of(&self) -> crate::TypeOf {
        crate::TypeOf::$value
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
