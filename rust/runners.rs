//! Runners _run_ a connection directly in Rust
//!
//! While this library exposes as many endpoints as necessary, runners keep
//! the processing of connections (requests and responses) in native-land as
//! much as possible, to minimize the time spent jumping from JavaScript code
//! to native code.

use crate::connection::Connection;
use crate::conninfo::Conninfo;
use crate::debug::*;
use crate::errors::*;
use neon::prelude::*;
use std::sync::Arc;

/* ========================================================================== *
 * STANDARD RUNNER: asynchronous _without_ pipelining                         *
 * ========================================================================== */

/// Simple struct wrapping a [`Connection`].
///
pub struct StandardRunner {
  id: usize,
  pub connection: Arc<Connection>,
}

debug_self!(StandardRunner, id);

impl From::<Connection> for StandardRunner {
  fn from(connection: Connection) -> Self {
    debug_create!(Self { id: debug_id(), connection: Arc::new(connection) })
  }
}

impl Finalize for StandardRunner {
  fn finalize<'a, C: Context<'a>>(self, _: &mut C) {
    debug!("Finalizing {:?}", self.connection);
    drop(self)
  }
}

/// Makes a new connection to the database server using using either an optional
/// connection string (DSN), or an object with the connection parameters.
///
pub fn run_standard(mut cx: FunctionContext) -> JsResult<JsPromise> {
  let options = cx.argument::<JsValue>(0)?;
  let callback = cx.argument::<JsFunction>(1)?;

  let info = {
    if let Ok(_) = options.downcast::<JsUndefined, _>(&mut cx) {
      Ok(Conninfo::default())
    } else if let Ok(_) = options.downcast::<JsNull, _>(&mut cx) {
      Ok(Conninfo::default())
    } else if let Ok(string) = options.downcast::<JsString, _>(&mut cx) {
      Conninfo::try_from(string.value(&mut cx)).or_throw(&mut cx)
    } else {
      let object = options.downcast_or_throw::<JsObject, _>(&mut cx)?;
      Conninfo::from_js_object(&mut cx, object)
    }
  }?;

  let promise = cx.task( || {
    let connection = Connection::try_from(info)?;

    connection.pq_setnonblocking(true)?;
    match connection.pq_isnonblocking() {
      false => Err("Unable to set non-blocking status".into()),
      true => Ok(connection),
    }
  }).promise(move | mut cx, result: PQResult<Connection> | {
    let connection = result.or_throw(&mut cx)?;
    Ok(cx.boxed(StandardRunner::from(connection)))
  });

  Ok(promise)
}
