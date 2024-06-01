//! Wrap LibPQ's own `pgNotify` struct.

use crate::errors::*;
use crate::ffi::*;
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
