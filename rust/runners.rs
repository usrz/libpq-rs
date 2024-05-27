//! Runners _run_ a connection directly in Rust
//!
//! While this library exposes as many endpoints as necessary, runners keep
//! the processing of connections (requests and responses) in native-land as
//! much as possible, to minimize the time spent jumping from JavaScript code
//! to native code.

use crate::connection::PQConnection;
use crate::conninfo::PQConninfo;
use crate::debug::*;
use crate::errors::*;
use neon::prelude::*;
use std::sync::Arc;
use crate::bindings::JSProcessor;

/* ========================================================================== *
 * PLAIN RUNNER: asynchronous _without_ pipelining                            *
 * ========================================================================== */

/// Simple struct wrapping a [`Connection`].
///
pub struct PlainRunner {
  id: usize,
  pub connection: Arc<PQConnection>,
}

debug_self!(PlainRunner, id);

impl From::<PQConnection> for PlainRunner {
  fn from(connection: PQConnection) -> Self {
    debug_create!(Self { id: debug_id(), connection: Arc::new(connection) })
  }
}

impl Finalize for PlainRunner {
  fn finalize<'a, C: Context<'a>>(self, _: &mut C) {
    debug!("Finalizing {:?}", self.connection);
    drop(self)
  }
}

/// Makes a new connection to the database server using using either an optional
/// connection string (DSN), or an object with the connection parameters.
///
pub fn plain_create(mut cx: FunctionContext) -> JsResult<JsPromise> {
  let options = cx.argument::<JsValue>(0)?;

  let info = {
    if let Ok(_) = options.downcast::<JsUndefined, _>(&mut cx) {
      Ok(PQConninfo::default())
    } else if let Ok(_) = options.downcast::<JsNull, _>(&mut cx) {
      Ok(PQConninfo::default())
    } else if let Ok(string) = options.downcast::<JsString, _>(&mut cx) {
      PQConninfo::try_from(string.value(&mut cx)).or_throw(&mut cx)
    } else {
      let object = options.downcast_or_throw::<JsObject, _>(&mut cx)?;
      PQConninfo::from_js_object(&mut cx, object)
    }
  }?;

  let callback = cx.argument::<JsFunction>(1)?;
  let processor = Box::new(JSProcessor::new(&mut cx, callback));

  let promise = cx.task( || {
    let connection = PQConnection::try_from(info)?;

    connection.pq_set_notice_processor(processor);

    connection.pq_setnonblocking(true)?;
    match connection.pq_isnonblocking() {
      false => Err("Unable to set non-blocking status".into()),
      true => Ok(connection),
    }
  }).promise(move | mut cx, result: PQResult<PQConnection> | {
    let connection = result.or_throw(&mut cx)?;
    Ok(cx.boxed(PlainRunner::from(connection)))
  });

  Ok(promise)
}
