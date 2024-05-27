//! Wrap LibPQ's own `pgNotify` struct.

use crate::errors::*;
use crate::ffi;

pub struct PQNotification {
  pub channel: String,
  pub be_pid: i32,
  pub extra: Option<String>,
}

impl TryFrom<*mut pq_sys::pgNotify> for PQNotification {
  type Error = PQError;

  fn try_from(value: *mut pq_sys::pgNotify) -> PQResult<Self> {
    let foo: &pq_sys::pgNotify = unsafe { &*{value} };
    let channel = ffi::to_string(foo.relname)?;
    let extra = ffi::to_string_lossy(foo.extra);
    let be_pid = foo.be_pid;
    Ok(Self { channel, be_pid, extra })
  }
}
