//! LibPQ notice processing (logs).

use crate::debug::*;
use crate::ffi::*;
use std::fmt::Debug;
use std::os::raw::c_void;

/// Level of a LibPQ notice message
///
/// See [PG_DIAG_SEVERITY_NONLOCALIZED](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQRESULTERRORFIELD)
/// See [Reporting Errors and Messages](https://www.postgresql.org/docs/current/plpgsql-errors-and-messages.html#PLPGSQL-STATEMENTS-RAISE)
///
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PQNoticeSeverity {
  Debug = 0,
  Log = 1,
  Info = 2,
  Notice = 3,
  Warning = 4,
}

impl From<String> for PQNoticeSeverity {
  fn from(value: String) -> Self {
    match value.to_lowercase().trim() {
      "debug" => Self::Debug,
      "log" => Self::Log,
      "info" => Self::Info,
      "notice" => Self::Notice,
      "warning" => Self::Warning,
      _ => {
        debug!("Unknown severity from LibPQ: \"{}\"", value);
        Self::Warning
      },
    }
  }
}

/// This is our "shared" notice processor. It's a basic function that will be
/// passed to LibPQ and will be invoked with a [`PQNoticeProcessorWrapper`] pointer.
///
pub unsafe extern "C" fn shared_notice_processor(data: *mut c_void, result: *const pq_sys::pg_result) {
  // Extract error message and status
  let (message_ptr, severity_ptr) = unsafe {
    let message = pq_sys::PQresultErrorMessage(result);
    let severity = pq_sys::PQresultErrorField(result, pq_sys::PG_DIAG_SEVERITY_NONLOCALIZED.into());

    (message, severity)
  };

  // This *will* return on PostgreSQL 9.5 and earlier...
  let severity = match to_string_lossy(severity_ptr) {
    Some(string) => PQNoticeSeverity::from(string),
    None => return,
  };

  // Trim out the message, if we have one
  let trimmed = match to_string_lossy(message_ptr) {
    Some(string) => string.trim().to_string(),
    None => return,
  };

  // Ignore empty messages and attempt to remove the prefix on others
  let message = match trimmed.as_str() {
    "" => return, // no empty strings
    trimmed => {
      // Looking at PostgreSQL sources, colon with two spaces _always_ separates
      // the localized level from the rest of the message
      match trimmed.split_once(":  ") {
        Some((_, message)) => message.to_string(),
        None => trimmed.to_string(),
      }
    }
  };


  debug!("Message from shared notice processor: [{:?}] {}", severity, message);

  // Convert our "data" pointer into a pointer to Connection and notify
  let wrapper = data as *mut PQNoticeProcessorWrapper;
  let this = unsafe { &*(wrapper) };
  this.process_notice(severity, message.to_string());
}

/// The trait that defines a processor of notice events from LibPQ.
///
pub trait PQNoticeProcessor: Debug {
  fn process_notice(&self, severity: PQNoticeSeverity, message: String) -> ();
}

/// Wrap a [`PQNoticeProcessor`] trait to safely decouple LibPQ's "extern C"
/// function into a Rust object.
///
/// Maybe there's a better way to handle this (and we can just get rid of this
/// wrapper altogether) but so far I haven't thought of a better way...
///
pub struct PQNoticeProcessorWrapper {
  id: usize,
  notice_processor: Box<dyn PQNoticeProcessor>
}

debug_self!(PQNoticeProcessorWrapper, id);

impl From::<Box<dyn PQNoticeProcessor>> for PQNoticeProcessorWrapper {
  fn from(notice_processor: Box<dyn PQNoticeProcessor>) -> Self {
    debug_create!(PQNoticeProcessorWrapper{ id: debug_id(), notice_processor })
  }
}

impl PQNoticeProcessorWrapper {
  fn process_notice(&self, severity: PQNoticeSeverity, message: String) -> () {
    debug!("Message from notice processor wrapper: {}", message);
    self.notice_processor.process_notice(severity, message);
  }
}

impl Drop for PQNoticeProcessorWrapper {
  fn drop(&mut self) {
    debug_drop!(self);
  }
}

/// The default notice processor simply dumps notices to the console...
///
pub struct PQDefaultNoticeProcessor {
  id: usize,
}

debug_self!(PQDefaultNoticeProcessor, id);

impl PQDefaultNoticeProcessor {
  pub fn new() -> Self {
    debug_create!(PQDefaultNoticeProcessor { id: debug_id() })
  }
}

impl PQNoticeProcessor for PQDefaultNoticeProcessor {
  fn process_notice(&self, severity: PQNoticeSeverity, message: String) -> () {
    println!(">>> from Postgres [{:?}] {}", severity, message);
  }
}

impl Drop for PQDefaultNoticeProcessor {
  fn drop(&mut self) {
    debug_drop!(self);
  }
}
