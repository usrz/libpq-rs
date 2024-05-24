use crate::sys::*;
use neon::prelude::*;
use std::os::raw::c_char;

#[derive(Debug)]
pub struct ConnInfo {
  values: Vec<(String, String)>,
}

impl TryFrom<&str> for ConnInfo {
  type Error = String;

  /// Create a [`ConnInfo`] struct from a PostgreSQL connection string (DSN).
  ///
  /// See [`PQconninfoParse`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFOPARSE)
  ///
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    unsafe {
      let str = value.as_bytes().as_ptr() as *const c_char;
      let mut err = std::ptr::null_mut();
      let raw = pq_sys::PQconninfoParse(str, &mut err);

      if raw.is_null() {
        if err.is_null() {
          Err("Unknown error parsing DSN string".to_string())
        } else {
          let msg = utils::to_string(err)?;
          Err(format!("Error parsing DSN string: {}", msg))
        }
      } else {
        let info = Self::try_from(raw)?;
        pq_sys::PQconninfoFree(raw);
        Ok(info)
      }
    }
  }
}

impl TryFrom<String> for ConnInfo {
  type Error = String;

  /// Create a [`ConnInfo`] struct from a PostgreSQL connection string (DSN).
  ///
  /// See [`PQconninfoParse`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFOPARSE)
  ///
  fn try_from(value: String) -> Result<Self, Self::Error> {
    ConnInfo::try_from(value.as_str())
  }
}

impl TryFrom<*mut pq_sys::_PQconninfoOption> for ConnInfo {
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

          let key = utils::to_string((* ptr).keyword)?;
          let value = utils::to_string((* ptr).val)?;

          values.push((key, value));
        }
      }
    }

    Ok(Self { values })
  }
}

impl ConnInfo {
  /// Create a [`ConnInfo`] struct from LibPQ's own defaults.
  ///
  /// This panics if the structure returned by `PQconndefaults` can not be
  /// safely converted into a [`ConnInfo`] struct.
  ///
  /// See [`PQconndefaults`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNDEFAULTS)
  ///
  pub fn new() -> Result<Self, String> {
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
  pub fn from_jsobject<'a, C: Context<'a>>(
    cx: &mut C,
    object: Handle<JsObject>
  ) -> NeonResult<ConnInfo> {
    let keys = object
      .get_own_property_names(cx)?
      .to_vec(cx)?;

    let mut values = Vec::<(String, String)>::new();
    for k in keys {
      let key = k.downcast_or_throw::<JsString, _>(cx)?.value(cx);
      let v = object.get_value(cx, k)?;
      let value = v.downcast::<JsString, _>(cx)
        .or_else(|_| {
          let value_type = types::js_type_of(v, cx);
          let message = format!("Invalid parameter value of type \"{}\" for key \"{}\"", value_type, key);
          cx.throw_error(message)
        })?
        .value(cx);

      values.push((key, value));
    }

    Ok(Self { values })
  }

  /// Convert a [`ConnInfo`] struct into a JavaScript object.
  ///
  pub fn to_object<'a, C: Context<'a>>(
    &self,
    cx: &mut C,
  ) -> NeonResult<Handle<'a, JsObject>> {
    let object = cx.empty_object();

    for (key, value) in self.values.iter() {
      let k = cx.string(key);
      let v = cx.string(value);
      object.set(cx, k, v)?;
    };

    Ok(object)
  }

  pub fn iter(&self) -> std::slice::Iter<(String, String)> {
    let q: std::slice::Iter<(String, String)> = self.values.iter();
    q
  }
}
