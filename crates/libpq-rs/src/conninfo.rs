//! Wrap LibPQ's own `PQconninfoOption`.

use crate::errors::*;
use crate::ffi::*;
use std::slice::Iter;

/// A wrapper for an array of LibPQ's own `PQconninfoOption`.
///
/// See [`PQconndefaults`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNDEFAULTS)
///
#[derive(Debug)]
pub struct PQConninfo {
  values: Vec<(String, String)>,
}

impl Default for PQConninfo {
  /// Create an empty [`Conninfo`] struct.
  ///
  /// LibPQ will apply its defaults anyway whenever opening a connection.
  ///
  fn default() -> Self {
    Self{ values: Vec::new() }
  }
}

impl TryFrom<&str> for PQConninfo {
  type Error = PQError;

  /// Create a [`Conninfo`] struct from a PostgreSQL connection string (DSN).
  ///
  /// See [`PQconninfoParse`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFOPARSE)
  ///
  fn try_from(value: &str) -> PQResult<Self> {
    let str = to_cstring(value);
    let mut err = std::ptr::null_mut();

    let raw = unsafe {
      pq_sys::PQconninfoParse(str.as_ptr(), &mut err)
    };

    if raw.is_null() {
      if err.is_null() {
        Err(format!("Unknown error parsing DSN string").into())
      } else {
        match to_string_lossy(err) {
          Some(msg) => Err(format!("Error parsing DSN string: {}", msg).into()),
          None => Err(format!("Unknown error parsing DSN string").into()),
        }
      }
    } else {
      let info = Self::try_from(raw)?;
      unsafe { pq_sys::PQconninfoFree(raw) };
      Ok(info)
    }
  }
}

impl TryFrom<String> for PQConninfo {
  type Error = PQError;

  /// Create a [`Conninfo`] struct from a PostgreSQL connection string (DSN).
  ///
  /// See [`PQconninfoParse`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFOPARSE)
  ///
  fn try_from(value: String) -> PQResult<Self> {
    Self::try_from(value.as_str())
  }
}

impl TryFrom<*mut pq_sys::_PQconninfoOption> for PQConninfo {
  type Error = PQError;

  /// Create a [`Conninfo`] struct from a LibPQ `PQconninfoOption` pointer.
  ///
  /// See [`PQconndefaults`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNDEFAULTS)
  ///
  fn try_from(raw: *mut pq_sys::_PQconninfoOption) -> PQResult<Self> {
    let mut values = Vec::<(String, String)>::new();

    unsafe {
      for x in 0.. {
        if (*raw.offset(x)).keyword.is_null() {
          break;
        } else {
          let ptr = raw.offset(x);

          let key = to_string_lossy((* ptr).keyword);
          let value = to_string_lossy((* ptr).val);

          if key.is_some() && value.is_some() {
            values.push((key.unwrap(), value.unwrap()));
          }
        }
      }
    }

    Ok(Self { values })
  }
}

impl PQConninfo {
  /// Create a [`Conninfo`] struct from LibPQ's own defaults.
  ///
  /// This panics if the structure returned by `PQconndefaults` can not be
  /// safely converted into a [`Conninfo`] struct.
  ///
  /// See [`PQconndefaults`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNDEFAULTS)
  ///
  pub fn from_libpq_defaults() -> PQResult<Self> {
    unsafe {
      let raw = pq_sys::PQconndefaults();
      Self::try_from(raw)
        .map(| info | {
          pq_sys::PQconninfoFree(raw);
          info
        })
        .map_err(|msg| {
          pq_sys::PQconninfoFree(raw);
          format!("Unable to access LibPQ defaults: {}", msg).into()
        })
    }
  }

  /// Iterate into a [`Conninfo`]'s own tuples.
  ///
  pub fn iter(&self) -> Iter<(String, String)> {
    self.values.iter()
  }
}
