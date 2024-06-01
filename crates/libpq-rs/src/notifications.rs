//! Wrap LibPQ's own `pgNotify` struct.

use crate::errors::*;
use crate::ffi::*;
// use neon::prelude::*;
use std::fmt::Debug;

#[derive(Debug)]
pub struct PQNotification {
  pub channel: String,
  pub be_pid: i32,
  pub extra: Option<String>,
}

impl TryFrom<*mut pq_sys::pgNotify> for PQNotification {
  type Error = PQError;

  fn try_from(pointer: *mut pq_sys::pgNotify) -> PQResult<Self> {
    let notification: &pq_sys::pgNotify = unsafe { &*{pointer} };

    let channel = to_string(notification.relname)?;
    let be_pid = notification.be_pid;

    let extra = match notification.extra.is_null() {
      false => Some(to_string(notification.extra)?),
      true => None,
    };

    Ok(Self { channel, be_pid, extra })
  }
}

impl PQNotification {
  // / Converts a [`PQNotification`] struct into a JavaScript object.
  // /
  // pub fn to_js_object<'a, C: Context<'a>>(
  //   &self,
  //   cx: &mut C,
  // ) -> NeonResult<Handle<'a, JsObject>> {
  //   let object = cx.empty_object();

  //   let channel = cx.string(&self.channel);
  //   object.set(cx, "channel", channel)?;

  //   let be_pid = cx.number(self.be_pid);
  //   object.set(cx, "be_pid", be_pid)?;

  //   if let Some(extra) = &self.extra {
  //     let extra = cx.string(extra);
  //     object.set(cx, "extra", extra)?;
  //   }

  //   Ok(object)
  // }
}
