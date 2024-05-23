use crate::{connection::connection_arg_0, sys::types::{js_type_of, JsTypeOf}};
use neon::prelude::*;
use polling::{ Event, Events, Poller };
use std::{
  io::Error,
  os::fd::BorrowedFd,
  time::Duration
};

fn poll(mut cx: FunctionContext, interest: Event) -> JsResult<JsPromise> {
  let conn = connection_arg_0!(cx);

  let timeout = match cx.argument_opt(1) {
    None => Ok(None),
    Some(timeout) => {
      match js_type_of(timeout, &mut cx) {
        JsTypeOf::JsNull |
        JsTypeOf::JsUndefined => Ok(None),
        JsTypeOf::JsNumber => {
          let milliseconds = timeout.downcast_or_throw::<JsNumber, _>(&mut cx)?;
          let duration = Duration::from_millis(milliseconds.value(&mut cx) as u64);
          Ok(Some(duration))
        },
        _ => {
          let ptype = js_type_of(timeout, &mut cx);
          cx.throw_error(format!("Invalid argument (1) of type \"{}\"", ptype))
        }
      }
    }
  }?;

  let fd = conn.pq_socket();

  let promise = cx.task(move || -> Result<(), Error> {
    let poller = Poller::new()?;
    let mut events = Events::new();

    let source = unsafe {
      let source = BorrowedFd::borrow_raw(fd);
      poller.add(&source, interest)?;
      source
    };

    poller.wait(&mut events, timeout)?;
    poller.delete(&source)?;

    Ok(())
  }).promise(move | mut cx, result | {
    match result {
      Err(error) => cx.throw_error(format!("Error polling: {}", error.to_string())),
      Ok(_) => Ok(cx.undefined()),
    }
  });

  Ok(promise)
}

pub fn poll_can_write(cx: FunctionContext) -> JsResult<JsPromise> {
  poll(cx, Event::writable(7))
}

pub fn poll_can_read(cx: FunctionContext) -> JsResult<JsPromise> {
  poll(cx, Event::readable(8))
}
