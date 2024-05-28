//! Runners _run_ a connection directly in Rust
//!
//! While this library exposes as many endpoints as necessary, runners keep
//! the processing of connections (requests and responses) in native-land as
//! much as possible, to minimize the time spent jumping from JavaScript code
//! to native code.

use crate::bindings::JSProcessor;
use crate::connection::PQConnection;
use crate::connection::PQPollingInterest;
use crate::conninfo::PQConninfo;
use crate::debug::*;
use crate::errors::*;
use neon::prelude::*;
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;

/* ========================================================================== *
 * PLAIN RUNNER: asynchronous _without_ pipelining                            *
 * ========================================================================== */

pub struct PlainRequest {
  query: String,
  params: Option<Vec<String>>,
  callback: Root<JsFunction>,
  single_row: bool,
}

impl PlainRequest {
  pub fn into(self) -> (String, Option<Vec<String>>, Root<JsFunction>, bool) {
    (self.query, self.params, self.callback, self.single_row)
  }
}

/// Simple struct wrapping a [`Connection`].
///
pub struct PlainRunner {
  id: usize,
  sender: mpsc::Sender<PlainRequest>,
}

debug_self!(PlainRunner, id);

impl Finalize for PlainRunner {
  fn finalize<'a, C: Context<'a>>(self, _: &mut C) {
    debug!("Finalizing {:?}", self);
    drop(self)
  }
}

impl PlainRunner {
  pub fn new<'a, C: Context<'a>>(
    cx: &mut C,
    connection: PQConnection,
  ) -> Self {
    let id = debug_id();
    let mut channel = cx.channel();
    channel.unref(cx);

    let (sender, receiver) = mpsc::channel::<PlainRequest>();

    thread::spawn(move || {
      let end: PQError = loop {

        // ===== WRITE REQUEST =================================================

        let request = match receiver.recv() {
          Ok(request) => request,
          Err(_) => break "Sender disconnected".into(),
        };

        let (
          query,
          params,
          callback,
          single_row,
        ) = request.into();

        // Need to write our callback in an Arc: we use it multiple times, and
        // it may outlive this thread, we can close the connection _before_ the
        // last result had time to get back to JavaScript land (channels)...
        let callback = Arc::new(callback);

        // Wait until we _can_ write...
        if let Err(error) = connection.poll(PQPollingInterest::Writable, None) {
          break error;
        }

        // Send the request once we can write
        if let Err(error) = match params {
          None => connection.pq_send_query(query),
          Some(params) => connection.pq_send_query_params(query, params),
        } {
          break error;
        }

        // Single row mode
        if single_row { connection.pq_set_single_row_mode(); }

        // Wait until the request is flushed
        if let Err(error) = loop {
          match connection.pq_flush() {
            Err(err) => break Err(err), // error in "pq_flush"
            Ok(true) => break Ok(()), // break, we're flushed
            Ok(false) => (), // don't break, poll!
          }

          // TODO: NOTIFICATIONS

          if let Err(error) = connection.poll(PQPollingInterest::Writable, None) {
            break Err(error);
          }
        } {
          break error;
        };

        // ===== READ RESPONSE =================================================

        let done: Result<(), PQError> = loop {
          // Wait until we _can_ read...
          if let Err(error) = connection.poll(PQPollingInterest::Readable, None) {
            break Err(error);
          }

          if let Err(error) = connection.pq_consume_input() {
            break Err(error); // error out in case "pq_consume_input" errs
          }

          // TODO: NOTIFICATIONS

          // One more loop, call "pq_is_busy" -> "pq_get_result" until
          let more = loop {

            // If "pq_get_result" would block, loop again and poll connection
            if connection.pq_is_busy() {
              break true; // here "true" will put us back to "poll" for reading
            }

            // We can safely get the result _without_ blocking
            let result = connection.pq_get_result();
            let more = result.is_some(); // gets moved out by channel

            // Invoke our callback, with the result or undefined...
            let callback = callback.clone();

            // TODO: notify the callback of the result, and poll again
            channel.send(move |mut cx| {
              let callback = callback.to_inner(&mut cx);
              let null = cx.null();

              cx.try_catch(move |cx| {
                // Invoke with the result object or undefined?
                let value = match result {
                  Some(result) => result.to_js_object(cx)?.as_value(cx),
                  None => null.as_value(cx),
                };

                // Invoke the callback
                callback.exec(cx, null, vec![value])

              }).or_else(|error| {
                cx.try_catch(move |cx| {
                  let string = error.to_string(cx)?.value(cx);
                  println!("Error invoking callback: {}", string);
                  Ok(())
                }).or_else(|_| {
                  println!("Error stringifying error from callback");
                  Ok(())
                })
              })
            });

            // Next step
            match more {
              true => continue, // result was not null, go back to check "pq_is_busy"
              false => break false, // result was null, this will go back at the beginning
            };
          };

          // Should we read some more data (poll, check busy, get result, ...)?
          match more {
            true => continue, // more data, back to polling on reads...
            false => break Ok(()),
          }
        };

        // Are we done with this request or did we error out?
        match done {
          Ok(_) => continue, // this will put us back reading the next query
          Err(error) => break error, // got an error, bail out
        }
      };

      debug!("Exiting loop in PlainRunner {{ id: {} }}: {}", id, end.message);
    });


    Self { id, sender }
  }

  pub fn enqueue(
    &self,
    query: String,
    params: Option<Vec<String>>,
    callback: Root<JsFunction>,
    single_row: bool,
  ) -> PQResult<()> {
    self.sender.send(PlainRequest {
      query,
      params,
      callback,
      single_row,
    }).map_err(|_| "Receiver disconnected".into())
  }
}

/// Makes a new connection to the database server using using either an optional
/// connection string (DSN), or an object with the connection parameters.
///
pub fn plain_create(mut cx: FunctionContext) -> JsResult<JsPromise> {
  let callback = cx.argument::<JsFunction>(0)?;
  let options = cx.argument_opt(1)
    .or(Some(cx.undefined().as_value(&mut cx)))
    .unwrap();

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
    let runner = PlainRunner::new(&mut cx, connection);
    Ok(cx.boxed(runner))
  });

  Ok(promise)
}

pub fn plain_query(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let runner = cx.argument::<JsBox<PlainRunner>>(0)?;
  let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);
  let command = cx.argument::<JsString>(2)?.value(&mut cx);

  let single_row = cx.argument_opt(3)
    .map(|value| value.downcast_or_throw::<JsBoolean, _>(&mut cx))
    .or(Some(Ok(cx.boolean(false)))) // default to "false"
    .unwrap()? // get the value (downscasted or default) or throw
    .value(&mut cx);

  runner.enqueue(command, None, callback, single_row)
    .or_throw(&mut cx)?;

  Ok(cx.undefined())
}

pub fn plain_query_params(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let runner = cx.argument::<JsBox<PlainRunner>>(0)?;
  let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);
  let command = cx.argument::<JsString>(2)?.value(&mut cx);

  let mut params = Vec::<String>::new();
  for param in cx.argument::<JsArray>(3)?.to_vec(&mut cx)? {
    let value = param.downcast_or_throw::<JsString, _>(&mut cx)?;
    let string = value.value(&mut cx);
    params.push(string);
  }

  let single_row = cx.argument_opt(4)
    .map(|value| value.downcast_or_throw::<JsBoolean, _>(&mut cx))
    .or(Some(Ok(cx.boolean(false)))) // default to "false"
    .unwrap()? // get the value (downscasted or default) or throw
    .value(&mut cx);

  runner.enqueue(command, Some(params), callback, single_row)
    .or_throw(&mut cx)?;

  Ok(cx.undefined())
}
