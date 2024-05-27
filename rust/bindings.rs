//! Straight bindings to LibPQ for Node/Neon.

use crate::connection::PQConnection;
use crate::connection::PQPollingInterest;
use crate::conninfo::PQConninfo;
use crate::debug::*;
use crate::errors::*;
use crate::notices::PQNoticeProcessor;
use crate::notices::PQNoticeSeverity;
use crate::notifications::PQNotificationProcessor;
use crate::response::PQResponse;
use neon::prelude::*;
use std::sync::Arc;
use std::time::Duration;

/* ========================================================================== *
 * STRUCTS                                                                    *
 * ========================================================================== */

/// Simple struct wrapping a [`Connection`].
///
pub struct JSConnection {
  id: usize,
  pub connection: Arc<PQConnection>,
}

debug_self!(JSConnection, id);

impl From::<PQConnection> for JSConnection {
  fn from(connection: PQConnection) -> Self {
    debug_create!(Self { id: debug_id(), connection: Arc::new(connection) })
  }
}

impl Finalize for JSConnection {
  fn finalize<'a, C: Context<'a>>(self, _: &mut C) {
    debug!("Finalizing {:?}", self.connection);
    drop(self)
  }
}

/* ========================================================================== */

/// Simple struct wrapping a [`PQResponse`]
///
pub struct  JSResponse {
  id: usize,
  pub response: Arc<PQResponse>,
}

debug_self!(JSResponse, id);

impl From::<PQResponse> for JSResponse {
  fn from(response: PQResponse) -> Self {
      Self { id: debug_id(), response: Arc::new(response) }
  }
}

impl Finalize for JSResponse {
  fn finalize<'a, C: Context<'a>>(self, _: &mut C) {
    debug!("Finalizing JsResponse {:?}", self.response);
    drop(self)
  }
}

/* ========================================================================== */

pub struct  JSProcessor {
  id: usize,
  channel: Channel,
  processor: Arc<Root<JsFunction>>,
}

debug_self!(JSProcessor, id);

impl JSProcessor {
  pub fn new<'a, C: Context<'a>>(cx: &mut C, processor: Handle<JsFunction>) -> Self {
    // Create a channel, and allow the Node event loop to exit
    let mut channel = cx.channel();
    channel.unref(cx);

    // Take full ownership of our function, we'll get rit of it in Drop!
    let rooted = processor.root(cx);
    let processor = Arc::new(rooted);

    debug_create!(Self { id: debug_id(), channel, processor })
  }
}

impl PQNoticeProcessor for JSProcessor {
  fn process_notice(&self, severity: PQNoticeSeverity, message: String) -> () {
    let proc: Arc<Root<JsFunction>> = self.processor.clone();

    self.channel.send(move |mut cx| {
      debug!("Message from JS processor: {}", message);

      let severity = match severity {
        PQNoticeSeverity::Debug => cx.string("debug").as_value(&mut cx),
        PQNoticeSeverity::Log => cx.string("log").as_value(&mut cx),
        PQNoticeSeverity::Info => cx.string("info").as_value(&mut cx),
        PQNoticeSeverity::Notice => cx.string("notice").as_value(&mut cx),
        PQNoticeSeverity::Warning => cx.string("warning").as_value(&mut cx),
      };

      let processor = proc.to_inner(&mut cx);
      let message = cx.string(message).as_value(&mut cx);
      let null = cx.null();

      processor.call(&mut cx, null, vec![severity, message]).and(Ok(()))
    });
  }
}

impl PQNotificationProcessor for JSProcessor {
  fn process_notice(&self, notification: crate::notifications::PQNotification) -> () {
    let proc: Arc<Root<JsFunction>> = self.processor.clone();

    self.channel.send(move |mut cx| {
      debug!("Notification from JS processor: {:?}", notification);

      let severity = cx.string("notification").as_value(&mut cx);

      let processor = proc.to_inner(&mut cx);
      let notification = notification.to_js_object(&mut cx)?.as_value(&mut cx);
      let null = cx.null();

      processor.call(&mut cx, null, vec![severity, notification]).and(Ok(()))
    });
  }
}

impl Drop for JSProcessor {
  fn drop(&mut self) {
    debug_drop!(self);
  }
}

/* ========================================================================== *
 * MACROS                                                                     *
 * ========================================================================== */

/// Convenience macro to extract from a [`Handle<<JsBox<JsConnection>>>`].
///
macro_rules! connection_arg_0 {
  ( $x:expr ) => { {
    let arg = $x.argument::<JsBox<JSConnection>>(0)?;
    arg.connection.clone()
  } };
}

/// Convenience macro to extract from a [`Handle<<JsBox<JsResponse>>>`].
///
macro_rules! response_arg_0 {
  ( $x:expr ) => { {
    let arg = $x.argument::<JsBox<JSResponse>>(0)?;
    arg.response.clone()
  } };
}

/* ========================================================================== *
 * CONNECTION                                                                 *
 * ========================================================================== */

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
      Ok(PQConninfo::default())
    } else if let Ok(_) = arg.downcast::<JsNull, _>(&mut cx) {
      Ok(PQConninfo::default())
    } else if let Ok(string) = arg.downcast::<JsString, _>(&mut cx) {
      PQConninfo::try_from(string.value(&mut cx)).or_throw(&mut cx)
    } else {
      let object = arg.downcast_or_throw::<JsObject, _>(&mut cx)?;
      PQConninfo::from_js_object(&mut cx, object)
    }
  }?;

  let promise = cx.task( || PQConnection::try_from(info))
    .promise(move | mut cx, result: PQResult<PQConnection> | {
      let connection = result.or_throw(&mut cx)?;
      Ok(cx.boxed(JSConnection::from(connection)))
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
  let processor = JSProcessor::new(&mut cx, processor);
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
  for param in cx.argument::<JsArray>(2)?.to_vec(&mut cx)? {
    let value = param.downcast_or_throw::<JsString, _>(&mut cx)?;
    let string = value.value(&mut cx);
    params.push(string);
  }

  connection.pq_send_query_params(command, params).or_throw(&mut cx)?;

  Ok(cx.undefined())
}

pub fn pq_get_result(mut cx: FunctionContext) -> JsResult<JsValue> {
  let connection = connection_arg_0!(cx);

  match connection.pq_get_result() {
    None => Ok(cx.undefined().as_value(&mut cx)),
    Some(response) => {
      let boxed = cx.boxed(JSResponse::from(response));
      Ok(boxed.as_value(&mut cx))
    },
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
fn poll(mut cx: FunctionContext, interest: PQPollingInterest) -> JsResult<JsPromise> {
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
  poll(cx, PQPollingInterest::Readable)
}

/// Wait until writes to the connection will not block.
///
pub fn poll_can_write(cx: FunctionContext) -> JsResult<JsPromise> {
  poll(cx, PQPollingInterest::Writable)
}

/* ========================================================================== *
 * RESPONSE                                                                   *
 * ========================================================================== */

pub fn pq_result_status(mut cx: FunctionContext) -> JsResult<JsString> {
  let response = response_arg_0!(cx);

  let status = response.pq_result_status();
  let string = format!("{:?}", status);

  Ok(cx.string(string))
}

pub fn pq_result_error_message(mut cx: FunctionContext) -> JsResult<JsValue> {
  let response = response_arg_0!(cx);

  match response.pq_result_error_message() {
    Some(string) => Ok(cx.string(string).as_value(&mut cx)),
    None => Ok(cx.undefined().as_value(&mut cx)),
  }
}

pub fn pq_cmd_status(mut cx: FunctionContext) -> JsResult<JsString> {
  let response = response_arg_0!(cx);
  Ok(cx.string(response.pq_cmd_status()))
}

pub fn pq_cmd_tuples(mut cx: FunctionContext) -> JsResult<JsNumber> {
  let response = response_arg_0!(cx);
  Ok(cx.number(response.pq_cmd_tuples()))
}

pub fn pq_ntuples(mut cx: FunctionContext) -> JsResult<JsNumber> {
  let response = response_arg_0!(cx);
  Ok(cx.number(response.pq_ntuples()))
}

pub fn pq_nfields(mut cx: FunctionContext) -> JsResult<JsNumber> {
  let response = response_arg_0!(cx);
  Ok(cx.number(response.pq_nfields()))
}

pub fn pq_fname(mut cx: FunctionContext) -> JsResult<JsValue> {
  let response = response_arg_0!(cx);
  let column = cx.argument::<JsNumber>(1)?.value(&mut cx) as i32;

  match response.pq_fname(column) {
    Some(string) => Ok(cx.string(string).as_value(&mut cx)),
    None => Ok(cx.undefined().as_value(&mut cx)),
  }
}

pub fn pq_ftype(mut cx: FunctionContext) -> JsResult<JsNumber> {
  let response = response_arg_0!(cx);
  let column = cx.argument::<JsNumber>(1)?.value(&mut cx) as i32;

  Ok(cx.number(response.pq_ftype(column)))
}

pub fn pq_getisnull(mut cx: FunctionContext) -> JsResult<JsBoolean> {
  let response = response_arg_0!(cx);
  let row = cx.argument::<JsNumber>(1)?.value(&mut cx) as i32;
  let col = cx.argument::<JsNumber>(2)?.value(&mut cx) as i32;

  Ok(cx.boolean(response.pq_getisnull(row, col)))
}

pub fn pq_getvalue(mut cx: FunctionContext) -> JsResult<JsValue> {
  let response = response_arg_0!(cx);
  let row = cx.argument::<JsNumber>(1)?.value(&mut cx) as i32;
  let col = cx.argument::<JsNumber>(2)?.value(&mut cx) as i32;

  let value = response.pq_getvalue(row, col).or_throw(&mut cx)?;

  match value {
    Some(string) => Ok(cx.string(string).as_value(&mut cx)),
    None => Ok(cx.undefined().as_value(&mut cx)),
  }
}

// ===== WRAPPING ==============================================================

pub fn unwrap_response(mut cx: FunctionContext) -> JsResult<JsObject> {
  let response = response_arg_0!(cx);
  response.to_js_object(&mut cx)
}
