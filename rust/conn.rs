use crate::connection::connection_arg_0;
use neon::prelude::*;

// CONNECTION: https://www.postgresql.org/docs/current/libpq-connect.html ======

pub fn pq_conninfo(mut cx: FunctionContext) -> JsResult<JsObject> {
  let connection = connection_arg_0!(cx);

  let info = connection.pq_conninfo()
    .or_else(|msg| cx.throw_error(msg))?;

  info.to_object(&mut cx)
}

// STATUS: https://www.postgresql.org/docs/current/libpq-status.html ===========

pub fn pq_status(mut cx: FunctionContext) -> JsResult<JsString> {
  let connection = connection_arg_0!(cx);

  let status = connection.pq_status();
  let string = format!("{:?}", status);

  Ok(cx.string(string))
}

pub fn pq_transaction_status(mut cx: FunctionContext) -> JsResult<JsString> {
  let connection = connection_arg_0!(cx);

  let status = connection.pq_transaction_status();
  let string = format!("{:?}", status);

  Ok(cx.string(string))
}

pub fn pq_server_version(mut cx: FunctionContext) -> JsResult<JsValue> {
  let connection = connection_arg_0!(cx);

  let version = connection.pq_server_version();
  match version {
    Some(version) => Ok(cx.string(version).as_value(&mut cx)),
    None => Ok(cx.undefined().as_value(&mut cx))
  }
}

pub fn pq_error_message(mut cx: FunctionContext) -> JsResult<JsValue> {
  let connection = connection_arg_0!(cx);

  let message = connection.pq_error_message();
  match message {
    Some(message) => Ok(cx.string(message).as_value(&mut cx)),
    None => Ok(cx.undefined().as_value(&mut cx))
  }
}

pub fn pq_socket(mut cx: FunctionContext) -> JsResult<JsNumber> {
  let connection = connection_arg_0!(cx);

  let socket = connection.pq_socket();
  Ok(cx.number(socket))
}

pub fn pq_backend_pid(mut cx: FunctionContext) -> JsResult<JsNumber> {
  let connection = connection_arg_0!(cx);

  let pid = connection.pq_backend_pid();
  Ok(cx.number(pid))
}

pub fn pq_ssl_in_use(mut cx: FunctionContext) -> JsResult<JsBoolean> {
  let connection = connection_arg_0!(cx);

  let ssl = connection.pq_ssl_in_use();
  Ok(cx.boolean(ssl))
}

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

// ASYNC https://www.postgresql.org/docs/current/libpq-async.html ==============

pub fn pq_consume_input(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let connection = connection_arg_0!(cx);

  connection.pq_consume_input()
    .or_else(|msg| cx.throw_error(msg))?;

    Ok(cx.undefined())
}

pub fn pq_is_busy(mut cx: FunctionContext) -> JsResult<JsBoolean> {
  let connection = connection_arg_0!(cx);

  let result = connection.pq_is_busy();

  Ok(cx.boolean(result))
}

pub fn pq_setnonblocking(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let connection = connection_arg_0!(cx);
  let nonblocking = cx.argument::<JsBoolean>(1)?.value(&mut cx);

  connection.pq_setnonblocking(nonblocking)
    .or_else(|msg| cx.throw_error(msg))?;

  Ok(cx.undefined())
}

pub fn pq_isnonblocking(mut cx: FunctionContext) -> JsResult<JsBoolean> {
  let connection = connection_arg_0!(cx);

  let result = connection.pq_isnonblocking();

  Ok(cx.boolean(result))
}

pub fn pq_flush(mut cx: FunctionContext) -> JsResult<JsBoolean> {
  let connection = connection_arg_0!(cx);

  let result = connection.pq_flush()
    .or_else(|msg| cx.throw_error(msg))?;

  Ok(cx.boolean(result))
}
