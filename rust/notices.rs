//! LibPQ notice processing.

use crate::debug;
use crate::debug_create;
use crate::debug_drop;
use crate::debug_id;
use crate::debug_self;
use crate::ffi;
use std::fmt::Debug;
use std::os::raw::c_char;
use std::os::raw::c_void;

/// This is our "shared" notice processor. It's a basic function that will be
/// passed to LibPQ and will be invoked with a `NoticeProcessorWrapper` pointer.
///
pub unsafe extern "C" fn shared_notice_processor(data: *mut c_void, message: *const c_char) {
  let string = match ffi::to_string_lossy(message) {
    Some(string) => string.trim().to_string(),
    None => return,
  };

  debug!("Message from shared notice processor: {}", string);

  // Convert our "data" pointer into a pointer to Connection and notify
  let wrapper = data as *mut NoticeProcessorWrapper;
  let this = unsafe { &*(wrapper) };
  this.process_notice(string);
}

/// The trait that defines a processor of notification events from LibPQ.
///
pub trait NoticeProcessor: Debug {
  fn process_notice(&self, message: String) -> ();
}

/// Wrap a [`NoticeProcessor`] trait to safely decouple LibPQ's "extern C"
/// function into a Rust object.
///
/// Maybe there's a better way to handle this (and we can just get rid of this
/// wrapper altogether) but so far I haven't thought of a better way...
///
pub struct NoticeProcessorWrapper {
  id: usize,
  notice_processor: Box<dyn NoticeProcessor>
}

debug_self!(NoticeProcessorWrapper, id);

impl From::<Box<dyn NoticeProcessor>> for NoticeProcessorWrapper {
  fn from(notice_processor: Box<dyn NoticeProcessor>) -> Self {
    debug_create!(NoticeProcessorWrapper{ id: debug_id(), notice_processor })
  }
}

impl NoticeProcessorWrapper {
  fn process_notice(&self, message: String) -> () {
    debug!("Message from notice processor wrapper: {}", message);
    self.notice_processor.process_notice(message);
  }
}

impl Drop for NoticeProcessorWrapper {
  fn drop(&mut self) {
    debug_drop!(self);
  }
}

/// The default notice processor simply dumps notices to the console...
///
pub struct DefaultNoticeProcessor {
  id: usize,
}

debug_self!(DefaultNoticeProcessor, id);

impl DefaultNoticeProcessor {
  pub fn new() -> Self {
    debug_create!(DefaultNoticeProcessor { id: debug_id() })
  }
}

impl NoticeProcessor for DefaultNoticeProcessor {
  fn process_notice(&self, message: String) -> () {
    println!(">>> from Postgres >>> {}", message);
  }
}

impl Drop for DefaultNoticeProcessor {
  fn drop(&mut self) {
    debug_drop!(self);
  }
}
