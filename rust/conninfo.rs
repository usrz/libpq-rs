use crate::sys::*;
use neon::prelude::*;
use std::os::raw::c_char;

#[derive(Debug)]
pub struct ConnInfo {
  values: Vec<(String, String)>,
}

impl ConnInfo {

  pub unsafe fn from_raw(raw: *mut pq_sys::_PQconninfoOption) -> Result<ConnInfo, String> {
    let mut values = Vec::<(String, String)>::new();

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

    Ok(Self { values })
  }

  pub fn from_defaults() -> Result<ConnInfo, String> {
    unsafe {
      let raw = pq_sys::PQconndefaults();
      let info = Self::from_raw(raw);
      pq_sys::PQconninfoFree(raw);
      info
    }
  }

  pub fn from_str(dsn: &str) -> Result<ConnInfo, String> {
    unsafe {
      let str = dsn.as_bytes().as_ptr() as *const c_char;
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
        let info = Self::from_raw(raw)?;
        pq_sys::PQconninfoFree(raw);
        Ok(info)
      }
    }
  }

  pub fn from_object<'a, C: Context<'a>>(
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
