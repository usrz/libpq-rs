//! Connection-related functions

use crate::connection::connection_arg_0;
use neon::prelude::*;

// ===== CONNECTION ============================================================

/// Returns the connection options used by a live connection.
///
/// See [`Connection::pq_conninfo`][crate::connection::Connection::pq_conninfo]
/// See [`PQconninfo`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFO)
///
pub fn pq_conninfo(mut cx: FunctionContext) -> JsResult<JsObject> {
  let connection = connection_arg_0!(cx);

  let info = connection.pq_conninfo()
    .or_else(|msg| cx.throw_error(msg))?;

  info.to_object(&mut cx)
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

  let attributes = connection.pq_ssl_attributes()
    .or_else(|msg| cx.throw_error(msg))?;

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

  connection.pq_consume_input()
    .or_else(|msg| cx.throw_error(msg))?;

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

  connection.pq_setnonblocking(nonblocking)
    .or_else(|msg| cx.throw_error(msg))?;

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

  let result = connection.pq_flush()
    .or_else(|msg| cx.throw_error(msg))?;

  Ok(cx.boolean(result))
}
