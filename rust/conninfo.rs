use crate::ffi;
use neon::prelude::*;
use std::slice::Iter;

/// A wrapper for an array of LibPQ's own `PQconninfoOption`.
///
/// See [`PQconndefaults`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNDEFAULTS)
///
#[derive(Debug)]
pub struct Conninfo {
  values: Vec<(String, String)>,
}

impl Default for Conninfo {
  /// Create an empty [`ConnInfo`] struct.
  ///
  /// LibPQ will apply its defaults anyway whenever opening a connection.
  ///
  fn default() -> Self {
    Self{ values: Vec::new() }
  }
}

impl TryFrom<&str> for Conninfo {
  type Error = String;

  /// Create a [`ConnInfo`] struct from a PostgreSQL connection string (DSN).
  ///
  /// See [`PQconninfoParse`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFOPARSE)
  ///
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let str = ffi::to_cstring(value);
    let mut err = std::ptr::null_mut();

    let raw = unsafe {
      pq_sys::PQconninfoParse(str.as_ptr(), &mut err)
    };

    if raw.is_null() {
      if err.is_null() {
        Err("Unknown error parsing DSN string".to_string())
      } else {
        let msg = ffi::to_string(err)?;
        Err(format!("Error parsing DSN string: {}", msg))
      }
    } else {
      let info = Self::try_from(raw)?;
      unsafe { pq_sys::PQconninfoFree(raw) };
      Ok(info)
    }
  }
}

impl TryFrom<String> for Conninfo {
  type Error = String;

  /// Create a [`ConnInfo`] struct from a PostgreSQL connection string (DSN).
  ///
  /// See [`PQconninfoParse`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFOPARSE)
  ///
  fn try_from(value: String) -> Result<Self, Self::Error> {
    Self::try_from(value.as_str())
  }
}

impl TryFrom<*mut pq_sys::_PQconninfoOption> for Conninfo {
  type Error = String;

  /// Create a [`ConnInfo`] struct from a LibPQ `PQconninfoOption` pointer.
  ///
  /// See [`PQconndefaults`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNDEFAULTS)
  ///
  fn try_from(raw: *mut pq_sys::_PQconninfoOption) -> Result<Self, Self::Error> {
    let mut values = Vec::<(String, String)>::new();

    unsafe {
      for x in 0.. {
        if (*raw.offset(x)).keyword.is_null() {
          break;
        } else {
          let ptr = raw.offset(x);

          if (*ptr).val.is_null() {
            continue;
          }

          let key = ffi::to_string((* ptr).keyword)?;
          let value = ffi::to_string((* ptr).val)?;

          values.push((key, value));
        }
      }
    }

    Ok(Self { values })
  }
}

impl Conninfo {
  /// Create a [`ConnInfo`] struct from LibPQ's own defaults.
  ///
  /// This panics if the structure returned by `PQconndefaults` can not be
  /// safely converted into a [`ConnInfo`] struct.
  ///
  /// See [`PQconndefaults`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNDEFAULTS)
  ///
  pub fn from_libpq_defaults() -> Result<Self, String> {
    unsafe {
      let raw = pq_sys::PQconndefaults();
      Self::try_from(raw)
        .and_then(| info | {
          pq_sys::PQconninfoFree(raw);
          Ok(info)
        }).or_else(|msg| {
          pq_sys::PQconninfoFree(raw);
          Err(format!("Unable to access LibPQ defaults: {}", msg))
        })
    }
  }

  /// Create a [`ConnInfo`] struct from a JavaScript object.
  ///
  pub fn from_js_object<'a, C: Context<'a>>(
    cx: &mut C,
    object: Handle<JsObject>
  ) -> NeonResult<Conninfo> {
    let keys = object
      .get_own_property_names(cx)?
      .to_vec(cx)?;

    let mut values = Vec::<(String, String)>::new();
    for k in keys {
      let key = k.downcast_or_throw::<JsString, _>(cx)?.value(cx);
      let value = object.get_value(cx, k)?
        .downcast_or_throw::<JsString, _>(cx)?
        .value(cx);
      values.push((key, value));
    }

    Ok(Self { values })
  }

  /// Convert a [`ConnInfo`] struct into a JavaScript object.
  ///
  pub fn to_js_object<'a, C: Context<'a>>(
    &self,
    cx: &mut C,
  ) -> NeonResult<Handle<'a, JsObject>> {
    let object = cx.empty_object();

    for (key, value) in self.iter() {
      let k = cx.string(key);
      let v = cx.string(value);
      object.set(cx, k, v)?;
    };

    Ok(object)
  }

  /// Iterate into a [`ConnInfo`]'s own tuples.
  ///
  pub fn iter(&self) -> Iter<(String, String)> {
    self.values.iter()
  }
}
