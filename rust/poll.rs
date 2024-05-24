use std::time::Duration;
use crate::connection::PollingInterest;

use neon::prelude::*;
use crate::connect::ArcConnection;
// use std::{sync::Arc, time::Duration};


fn poll(mut cx: FunctionContext, interest: PollingInterest) -> JsResult<JsPromise> {
  let aaa = cx.argument::<JsBox<ArcConnection>>(0)?.connection();

  let timeout = match cx.argument_opt(1) {
    None => Ok(None),
    Some(timeout) => {
      if let Ok(_) = timeout.downcast::<JsUndefined, _>(&mut cx) {
        Ok(None)
      } else if let Ok(_) = timeout.downcast::<JsNull, _>(&mut cx) {
        Ok(None)
      } else {
        let milliseconds = timeout.downcast_or_throw::<JsNumber, _>(&mut cx)?;
        let duration = Duration::from_millis(milliseconds.value(&mut cx) as u64);
        Ok(Some(duration))
      }
    }
  }?;

  let promise = cx.task( move || {
    aaa.poll(interest, timeout)
    // let w = conn.borrow();
    // conn.poll(PollingInterest::Readable, None)
  }).promise(move | mut cx, result | {
    match result {
      Err(error) => cx.throw_error(format!("Error polling: {}", error.to_string())),
      Ok(_) => Ok(cx.undefined()),
    }
  });

  Ok(promise)
}

pub fn poll_can_write(cx: FunctionContext) -> JsResult<JsPromise> {
  poll(cx, PollingInterest::Writable)
}

pub fn poll_can_read(cx: FunctionContext) -> JsResult<JsPromise> {
  poll(cx, PollingInterest::Readable)
}
