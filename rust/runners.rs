//! Runners _run_ a connection directly in Rust
//!
//! While this library exposes as many endpoints as necessary, runners keep
//! the processing of connections (requests and responses) in native-land as
//! much as possible, to minimize the time spent jumping from JavaScript code
//! to native code.

use neon::prelude::*;
use crate::connection::PQConnection;
use crate::conninfo::PQConninfo;
use crate::debug::*;
use crate::errors::*;
use std::sync::Arc;
use crate::bindings::JSProcessor;
use neon::types::Deferred;
use std::thread;
use std::sync::mpsc;
use crate::connection::PQPollingInterest;

/* ========================================================================== *
 * PLAIN RUNNER: asynchronous _without_ pipelining                            *
 * ========================================================================== */

pub struct PlainRequest {
  pub query: String,
  pub params: Option<Vec<String>>,
  pub callback: Root<JsFunction>,
  pub deferred: Deferred,
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
    let channel = cx.channel();

    let (sender, receiver) = mpsc::channel::<PlainRequest>();

    thread::spawn(move || {
      let end: PQError = loop {

        // ===== WRITE REQUEST =================================================

        let request = match receiver.recv() {
          Ok(request) => request,
          Err(_) => break "Sender disconnected".into(),
        };

        // Wait until we _can_ write...
        connection.poll(PQPollingInterest::Writable, None);

        // Send the request once we can write
        match request.params {
          None => connection.pq_send_query(request.query),
          Some(params) => connection.pq_send_query_params(request.query, params),
        };

        // Wait until the request is flushed
        let flushed = loop {
          match connection.pq_flush() {
            Ok(false) => connection.poll(PQPollingInterest::Writable, None),
            Ok(true) => break Ok(()), // break, we're flushed
            Err(err) => break Err(err), // error
          };
        };

        // Error on flushing? Break out!
        if let Err(error) = flushed {
          break error;
        }

        // ===== READ RESPONSE =================================================

        let done: Result<(), PQError> = loop {
          // Wait until we _can_ read...
          connection.poll(PQPollingInterest::Readable, None);

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
            match connection.pq_get_result() {
              Some(result) => {
                // channel.send(|mut cx| {
                //   let callback = request.callback.to_inner(&mut cx);
                //   let null = cx.null();
                //   let object = result.to_js_object(&mut cx);

                //   Ok(())

                // });
                // TODO: notify the callback of the result, and poll again
                continue; // run "pq_is_busy" -> "pq_get_result" again
              },

              None => {
                // TODO: resolve the promise with "undefined" and next request

                break false; // this will send us back to the next request
              }
            }
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
    deferred: Deferred,
    callback: Root<JsFunction>,
  ) -> PQResult<()> {
    self.sender.send(PlainRequest {
      query,
      params,
      deferred,
      callback,
    });

    Ok(())
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
    let runner = PlainRunner::new(&mut cx, connection);
    Ok(cx.boxed(runner))
  });

  Ok(promise)
}

pub fn plain_query_params(mut cx: FunctionContext) -> JsResult<JsPromise> {
  let runner = cx.argument::<JsBox<PlainRunner>>(0)?;

  let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);

  let command = cx.argument::<JsString>(2)?.value(&mut cx);

  let mut params = Vec::<String>::new();
  for param in cx.argument::<JsArray>(3)?.to_vec(&mut cx)? {
    let value = param.downcast_or_throw::<JsString, _>(&mut cx)?;
    let string = value.value(&mut cx);
    params.push(string);
  }

  let (deferred, promise) = cx.promise();

  runner.enqueue(command, Some(params), deferred, callback)
    .or_throw(&mut cx)?;

  Ok(promise)
}
