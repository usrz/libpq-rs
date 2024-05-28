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
use std::time::Duration;

/* ========================================================================== *
 * RUNNER: asynchronous _without_ pipelining                                  *
 * ========================================================================== */

/// Super simple struct to hold a "request" to postgres
///
struct RunnerRequest {
  query: String,
  params: Option<Vec<String>>,
  callback: Root<JsFunction>,
  single_row: bool,
}

impl RunnerRequest {
  pub fn into(self) -> (String, Option<Vec<String>>, Root<JsFunction>, bool) {
    (self.query, self.params, self.callback, self.single_row)
  }
}

/* ========================================================================== */

/// Our asynchronous runner of queries.
///
pub struct Runner {
  id: usize,
  sender: mpsc::Sender<RunnerRequest>,
}

debug_self!(Runner, id);

impl Finalize for Runner {
  fn finalize<'a, C: Context<'a>>(self, _: &mut C) {
    debug!("Finalizing {:?}", self);
    drop(self)
  }
}

impl Runner {
  pub fn new<'a, C: Context<'a>>(
    cx: &mut C,
    connection: PQConnection,
    callback: Handle<JsFunction>,
  ) -> Self {
    let id = debug_id();
    let mut channel = cx.channel();
    channel.unref(cx);

    // The callback is shared between our notice processor, and our thread for
    // send notifications from channels while processing requests
    let conn_callback = Arc::new(callback.root(cx));

    // Setup notice processing
    let processor = JSProcessor::new(channel.clone(), conn_callback.clone());
    connection.pq_set_notice_processor(Box::new(processor));

    // Setup our channel for enqueueing requests
    let (sender, receiver) = mpsc::channel::<RunnerRequest>();

    // The core of our process runs in a separate thread (with many many loops!)
    thread::spawn(move || {
      let this = format!("Runner {{ id: {}, thread: {:?} }}", id, thread::current().id());

      let end: PQError = 'request: loop {

        // ===== WRITE REQUEST =================================================

        let request = match receiver.recv() {
          Ok(request) => request,
          Err(_) => break 'request "Sender disconnected".into(),
        };

        debug!("Received request on {}", this);

        let (
          query,
          params,
          callback,
          single_row,
        ) = request.into();

        // Need to write our callback in an Arc: we use it multiple times, and
        // it may outlive this thread, we can close the connection _before_ the
        // last result had time to get back to JavaScript land (channels)...
        //
        // NOTE: Root<JsFunction> _is_ clonable (internally root _does_ use
        // Arcs), but we need a context to do so... In our case this is
        // unworkable, as we need to move it to our channel closure mutiple
        // times, and we have a context _only_ in the closure itself!
        let callback = Arc::new(callback);

        // Wait until we _can_ write...
        if let Err(error) = connection.poll(PQPollingInterest::Writable, None) {
          break 'request error;
        }

        // Send the request once we can write
        if let Err(error) = match params {
          None => connection.pq_send_query(query),
          Some(params) => connection.pq_send_query_params(query, params),
        } {
          break 'request error;
        }

        // Single row mode
        if single_row { connection.pq_set_single_row_mode(); }

        // Wait until the request is flushed
        'flush: loop {
          match connection.pq_flush() {
            Err(err) => break 'request err, // error in "pq_flush"
            Ok(true) => break 'flush, // break, we're flushed
            Ok(false) => (), // don't break, poll!
          }

          if let Err(error) = connection.poll(PQPollingInterest::Writable, None) {
            break 'request error;
          }
        };

        debug!("WE JUST WROTE, CAN WE WRITE AGAIN");
        if let Err(error) = connection.poll(PQPollingInterest::Writable, Some(Duration::from_secs(2))) {
          break 'request error;
        }
        debug!("WE JUST WROTE, CAN WE WRITE AGAIN PART 2");

        // ===== READ RESPONSE =================================================

        'response: loop {
          // Wait until we _can_ read...
          if let Err(error) = connection.poll(PQPollingInterest::Readable, None) {
            break 'request error;
          }

          if let Err(error) = connection.pq_consume_input() {
            break 'request error; // error out in case "pq_consume_input" errs
          }

          // TODO: NOTIFICATIONS

          // One more loop, call "pq_is_busy" -> "pq_get_result" until
          'partial: loop {

            // If "pq_get_result" would block, loop again and poll connection
            if connection.pq_is_busy() {
              continue 'response; // here "true" will put us back to "poll" for reading
            }

            // We can safely get the result _without_ blocking
            let result = connection.pq_get_result();
            let more = result.is_some(); // gets moved out by channel

            match &result {
              Some(result) => debug!("Received \"{:?}\" result on {}", result.pq_result_status(), this),
              None => debug!("Received final result on {}", this),
            };

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
              true => continue 'partial, // result was not null, go back to check "pq_is_busy"
              false => continue 'request, // result was null, this will go back at the beginning
            };
          };
        };
      };

      debug!("Exiting loop in {}: {}", this, end.message);
    });

    // Here we are!
    Self { id, sender }
  }

  /// Enqueue a request (a query, its parameters, the callback where to send
  /// the results and a flag indicating whether single row mode is active)
  ///
  pub fn enqueue(
    &self,
    query: String,
    params: Option<Vec<String>>,
    callback: Root<JsFunction>,
    single_row: bool,
  ) -> PQResult<()> {
    self.sender.send(RunnerRequest {
      query,
      params,
      callback,
      single_row,
    }).map_err(|_| "Receiver disconnected".into())
  }
}

/* ========================================================================== */

/// Makes a new connection to the database server using using either an optional
/// connection string (DSN), or an object with the connection parameters, and
/// returns a [`Runner`] executing queries.
///
pub fn runner_create(mut cx: FunctionContext) -> JsResult<JsPromise> {
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

  // Root the callback until the connection is established
  let rooted_callback = callback.root(&mut cx);

  // Asynchronously (but synchronously) open our connection in a task
  let promise = cx.task( || {
    let connection = PQConnection::try_from(info)?;

    connection.pq_setnonblocking(true)?;
    match connection.pq_isnonblocking() {
      false => Err("Unable to set non-blocking status".into()),
      true => Ok(connection),
    }
  }).promise(move | mut cx, result: PQResult<PQConnection> | {
    let connection = result.or_throw(&mut cx)?;
    let callback = rooted_callback.into_inner(&mut cx);

    let runner = Runner::new(&mut cx, connection, callback);
    Ok(cx.boxed(runner))
  });

  Ok(promise)
}

/// Send a straigh query (no parameters) to a [`Runner`].
///
pub fn runner_query(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let runner = cx.argument::<JsBox<Runner>>(0)?;
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

/// Send a parameterized query to a [`Runner`].
///
pub fn runner_query_params(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let runner = cx.argument::<JsBox<Runner>>(0)?;
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
