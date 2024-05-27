//! Connection-related functions

use crate::connection::Connection;
use crate::notices::NoticeProcessor;
use crate::connection::PollingInterest;
use crate::conninfo::Conninfo;
use crate::debug;
use crate::errors::*;
use neon::prelude::*;
use std::sync::Arc;
use std::time::Duration;

/* ========================================================================== *
 * STRUCTS                                                                    *
 * ========================================================================== */

/// Simple struct wrapping a [`Connection`].
///
pub struct JsConnection {
  pub connection: Arc<Connection>,
}

impl From::<Connection> for JsConnection {
  fn from(connection: Connection) -> Self {
    Self { connection: Arc::new(connection) }
  }
}

impl Finalize for JsConnection {
  fn finalize<'a, C: Context<'a>>(self, _: &mut C) {
    debug!("Finalizing JsConnection");
    drop(self)
  }
}

pub struct  JsNoticeProcessor {
  channel: Channel,
  processor: Arc<Root<JsFunction>>,
}

impl JsNoticeProcessor {
  fn new<'a, C: Context<'a>>(cx: &mut C, processor: Handle<JsFunction>) -> Self {
    // Create a channel, and allow the Node event loop to exit
    let mut channel = cx.channel();
    channel.unref(cx);

    // Take full ownership of our function, we'll get rit of it in Drop!
    let rooted = processor.root(cx);

    let processor = Arc::new(rooted);
    let weak = Arc::weak_count(&processor);
    let strong = Arc::strong_count(&processor);
    println!("~~~ Creatubg JS notice processor weak={} strong={}", weak, strong);

    JsNoticeProcessor{ channel, processor }
  }
}

impl NoticeProcessor for JsNoticeProcessor {
  fn process_notice(&self, message: String) -> () {
    let weak = Arc::weak_count(&self.processor);
    let strong = Arc::strong_count(&self.processor);
    println!("~~~ Cloning JS notice processor weak={} strong={}", weak, strong);

    let proc: Arc<Root<JsFunction>> = self.processor.clone();

    let weak = Arc::weak_count(&self.processor);
    let strong = Arc::strong_count(&self.processor);
    println!("~~~ Cloned JS notice processor weak={} strong={}", weak, strong);

    self.channel.send(move |mut cx| {
      debug!("Message from JS notice processor: {}", message);

      let processor = proc.to_inner(&mut cx);
      let string = cx.string(message).as_value(&mut cx);
      let null = cx.null();

      let result = processor.call(&mut cx, null, vec![string]).and(Ok(()));

      let weak = Arc::weak_count(&proc);
      let strong = Arc::strong_count(&proc);
      println!("~~~ JS notice processor references weak={} strong={}", weak, strong);

      result
    });
  }
}

impl Drop for JsNoticeProcessor {
  fn drop(&mut self) {
    let weak = Arc::weak_count(&self.processor);
    let strong = Arc::strong_count(&self.processor);
    println!("~~~ Dropping JS notice processor weak={} strong={}", weak, strong);
  }
}


/// Convenience macro to extract from a `Handle<<JsBox<ArcConnection>>>`.
///
macro_rules! connection_arg_0 {
  ( $x:expr ) => { {
    let arg = $x.argument::<JsBox<JsConnection>>(0)?;
    arg.connection.clone()
  } };
}

// ===== CONNECTION ============================================================

/// Makes a new connection to the database server using using either an optional
/// connection string (DSN), or an object with the connection parameters.
///
/// This function will _first_ convert the first argument into a [`Conninfo`]
/// struct (default, from string, or from an object), and then call LibPQ's own
/// `PQconnectdbParams` to establish the connection.
///
/// While connecting is performed in a _synchronous_ way, this function will
/// not block and return a `Promise` to the [`JsBox`] for the connection.
///
/// See [`PQconndefaults`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNDEFAULTS)
/// See [`PQconninfoParse`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFOPARSE)
/// See [`PQconnectdbParams`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTDBPARAMS)
///
pub fn pq_connectdb_params(mut cx: FunctionContext) -> JsResult<JsPromise> {
  let arg = cx.argument_opt(0)
    .unwrap_or(cx.undefined().as_value(&mut cx));

  let info = {
    if let Ok(_) = arg.downcast::<JsUndefined, _>(&mut cx) {
      Ok(Conninfo::default())
    } else if let Ok(_) = arg.downcast::<JsNull, _>(&mut cx) {
      Ok(Conninfo::default())
    } else if let Ok(string) = arg.downcast::<JsString, _>(&mut cx) {
      Conninfo::try_from(string.value(&mut cx)).or_throw(&mut cx)
    } else {
      let object = arg.downcast_or_throw::<JsObject, _>(&mut cx)?;
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
    Ok(cx.boxed(JsConnection::from(connection)))
  });

  Ok(promise)
}

/// Returns the connection options used by a live connection.
///
/// See [`Connection::pq_conninfo`][crate::connection::Connection::pq_conninfo]
/// See [`PQconninfo`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFO)
///
pub fn pq_conninfo(mut cx: FunctionContext) -> JsResult<JsObject> {
  let connection = connection_arg_0!(cx);

  let info = connection.pq_conninfo().or_throw(&mut cx)?;

  info.to_js_object(&mut cx)
}

/// Sets the current notice processor.
///
/// See [PQnoticeProcessor](https://www.postgresql.org/docs/current/libpq-notice-processing.html)
pub fn pq_set_notice_processor(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let connection = connection_arg_0!(cx);
  let processor = cx.argument::<JsFunction>(1)?;
  let processor = JsNoticeProcessor::new(&mut cx, processor);
  connection.pq_set_notice_processor(Box::new(processor));
  Ok(cx.undefined())
}

// ===== STATUS ================================================================

/// Returns the status of the connection.
///
/// See [`Connection::pq_status`][crate::connection::Connection::pq_status]
/// See [`PQstatus`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSTATUS)
///
pub fn pq_status(mut cx: FunctionContext) -> JsResult<JsString> {
  let connection = connection_arg_0!(cx);

  let status = connection.pq_status();
  let string = format!("{:?}", status);

  Ok(cx.string(string))
}

/// Returns the current in-transaction status of the server.
///
/// See [`Connection::pq_transaction_status`][crate::connection::Connection::pq_transaction_status]
/// See [`PQtransactionStatus`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQTRANSACTIONSTATUS)
///
pub fn pq_transaction_status(mut cx: FunctionContext) -> JsResult<JsString> {
  let connection = connection_arg_0!(cx);

  let status = connection.pq_transaction_status();
  let string = format!("{:?}", status);

  Ok(cx.string(string))
}

/// Returns the server version as a `String`.
///
/// See [`Connection::pq_server_version`][crate::connection::Connection::pq_server_version]
/// See [`PQserverVersion`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSERVERVERSION)
///
pub fn pq_server_version(mut cx: FunctionContext) -> JsResult<JsValue> {
  let connection = connection_arg_0!(cx);

  let version = connection.pq_server_version();
  match version {
    Some(version) => Ok(cx.string(version).as_value(&mut cx)),
    None => Ok(cx.undefined().as_value(&mut cx))
  }
}

/// Returns the error message most recently generated by an operation on the connection.
///
/// See [`Connection::pq_error_message`][crate::connection::Connection::pq_error_message]
/// See [`PQerrorMessage`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQERRORMESSAGE)
///
pub fn pq_error_message(mut cx: FunctionContext) -> JsResult<JsValue> {
  let connection = connection_arg_0!(cx);

  let message = connection.pq_error_message();
  match message {
    Some(message) => Ok(cx.string(message).as_value(&mut cx)),
    None => Ok(cx.undefined().as_value(&mut cx))
  }
}

/// Obtains the file descriptor number of the connection socket to the server.
///
/// See [`Connection::pq_socket`][crate::connection::Connection::pq_socket]
/// See [`PQsocket`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSOCKET)
///
pub fn pq_socket(mut cx: FunctionContext) -> JsResult<JsNumber> {
  let connection = connection_arg_0!(cx);

  let socket = connection.pq_socket();
  Ok(cx.number(socket))
}

/// Returns the process ID (PID) of the backend process handling this connection.
///
/// See [`Connection::pq_backend_pid`][crate::connection::Connection::pq_backend_pid]
/// See [`PQbackendPID`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQBACKENDPID)
///
pub fn pq_backend_pid(mut cx: FunctionContext) -> JsResult<JsNumber> {
  let connection = connection_arg_0!(cx);

  let pid = connection.pq_backend_pid();
  Ok(cx.number(pid))
}

/// Returns `true` if the connection uses SSL, `false` if not.
///
/// See [`Connection::pq_ssl_in_use`][crate::connection::Connection::pq_ssl_in_use]
/// See [`PQsslInUse`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSSLINUSE)
///
pub fn pq_ssl_in_use(mut cx: FunctionContext) -> JsResult<JsBoolean> {
  let connection = connection_arg_0!(cx);

  let ssl = connection.pq_ssl_in_use();
  Ok(cx.boolean(ssl))
}

/// Returns SSL-related information about the connection.
///
/// This returns a `JsObject` containing _only_ the key-value mappings for
/// non-null attributes.
///
/// See [`Connection::pq_ssl_attributes`][crate::connection::Connection::pq_ssl_attributes]
/// See [`PQsslAttribute`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSSLATTRIBUTE)
/// See [`PQsslAttributeNames`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSSLATTRIBUTENAMES)
///
pub fn pq_ssl_attributes(mut cx: FunctionContext) -> JsResult<JsObject> {
  let connection = connection_arg_0!(cx);

  let attributes = connection.pq_ssl_attributes().or_throw(&mut cx)?;

  let object = cx.empty_object();
  for (key, value) in attributes.iter() {
    let k = cx.string(key);
    let v = cx.string(value);
    object.set(&mut cx, k, v)?;
  }

  Ok(object)
}

// ===== ASYNC =================================================================

/// If input is available from the server, consume it.
///
/// See [`Connection::pq_consume_input`][crate::connection::Connection::pq_consume_input]
/// See [`PQconsumeInput`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQCONSUMEINPUT)
///
pub fn pq_consume_input(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let connection = connection_arg_0!(cx);

  connection.pq_consume_input().or_throw(&mut cx)?;
  Ok(cx.undefined())
}

/// Returns `true` if a command is busy, that is, `pq_get_result` would block
/// waiting for input. A `false` return indicates that `pq_get_result` can be
/// called with assurance of not blocking.
///
/// See [`Connection::pq_is_busy`][crate::connection::Connection::pq_is_busy]
/// See [`PQisBusy`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQISBUSY)
///
pub fn pq_is_busy(mut cx: FunctionContext) -> JsResult<JsBoolean> {
  let connection = connection_arg_0!(cx);

  let result = connection.pq_is_busy();

  Ok(cx.boolean(result))
}

/// Sets the nonblocking status of the connection.
///
/// See [`Connection::pq_setnonblocking`][crate::connection::Connection::pq_setnonblocking]
/// See [`PQsetnonblocking`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSETNONBLOCKING)
///
pub fn pq_setnonblocking(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let connection = connection_arg_0!(cx);
  let nonblocking = cx.argument::<JsBoolean>(1)?.value(&mut cx);

  connection.pq_setnonblocking(nonblocking).or_throw(&mut cx)?;

  Ok(cx.undefined())
}

/// Returns the nonblocking status of the database connection.
///
/// See [`Connection::pq_isnonblocking`][crate::connection::Connection::pq_isnonblocking]
/// See [`PQisnonblocking`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQISNONBLOCKING)
///
pub fn pq_isnonblocking(mut cx: FunctionContext) -> JsResult<JsBoolean> {
  let connection = connection_arg_0!(cx);

  let result = connection.pq_isnonblocking();

  Ok(cx.boolean(result))
}

/// Attempts to flush any queued output data to the server.
///
/// Returns `true` if successful (or if the send queue is empty), an error
/// if it failed for some reason, or `false` if it was unable to send all the
/// data in the send queue yet (this case can only occur if the connection is
/// nonblocking).
///
/// See [`Connection::pq_flush`][crate::connection::Connection::pq_flush]
/// See [`PQflush`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQFLUSH)
///
pub fn pq_flush(mut cx: FunctionContext) -> JsResult<JsBoolean> {
  let connection = connection_arg_0!(cx);

  let result = connection.pq_flush().or_throw(&mut cx)?;

  Ok(cx.boolean(result))
}

// ===== ASYNCHRONOUS OPERATIONS ===============================================

pub fn pq_send_query(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let connection = connection_arg_0!(cx);

  // todo maybe nicer types?
  let command = cx.argument::<JsString>(1)?.value(&mut cx);

  connection.pq_send_query(command).or_throw(&mut cx)?;

  Ok(cx.undefined())
}

pub fn pq_send_query_params(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let connection = connection_arg_0!(cx);

  // todo maybe nicer types?
  let command = cx.argument::<JsString>(1)?.value(&mut cx);

  // parameters
  let mut params = Vec::<String>::new();
  for i in 2.. cx.len() {
    let param = cx.argument::<JsString>(i)?.value(&mut cx);
    params.push(param);
  }

  connection.pq_send_query_params(command, params).or_throw(&mut cx)?;

  Ok(cx.undefined())
}

pub fn pq_get_result(mut cx: FunctionContext) -> JsResult<JsString> {
  let connection = connection_arg_0!(cx);

  match connection.pq_get_result() {
    Some(result) => Ok(cx.string(format!("RESULT STATUS {:?}", result.pq_result_status()))),
    None => Ok(cx.string("DONE"))
  }
}

// ===== SINGLE ROW MODE =======================================================

/// Select single-row mode for the currently-executing query.
///
/// See [`PQsetSingleRowMode`](https://www.postgresql.org/docs/current/libpq-single-row-mode.html#LIBPQ-PQSETSINGLEROWMODE)
///
pub fn pq_set_single_row_mode(mut cx: FunctionContext) -> JsResult<JsBoolean> {
  let connection = connection_arg_0!(cx);

  let result = connection.pq_set_single_row_mode();

  Ok(cx.boolean(result))
}

// ===== POLLING ===============================================================

/// Wait until reads from or writes to the connection will not block.
///
fn poll(mut cx: FunctionContext, interest: PollingInterest) -> JsResult<JsPromise> {
  let connection = connection_arg_0!(cx);

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
    println!("POLLING ON THREAD {:?}", std::thread::current().id());
    connection.poll(interest, timeout)
  }).promise(move | mut cx, result | {
    println!("POLLED RESOLVING ON THREAD {:?}", std::thread::current().id());
    match result {
      Err(error) => cx.throw_error(format!("Error polling: {}", error.to_string())),
      Ok(_) => Ok(cx.undefined()),
    }
  });

  Ok(promise)
}

/// Wait until reads to the connection will not block.
///
pub fn poll_can_read(cx: FunctionContext) -> JsResult<JsPromise> {
  poll(cx, PollingInterest::Readable)
}

/// Wait until writes to the connection will not block.
///
pub fn poll_can_write(cx: FunctionContext) -> JsResult<JsPromise> {
  poll(cx, PollingInterest::Writable)
}
