//! Wrap the LibPQ function in a slightly more convenient _struct_.

use crate::conninfo::ConnInfo;
use crate::sys::*;
use neon::prelude::*;

static ENCODING_KEY: &str = "client_encoding";
static ENCODING_VAL: &str = "UTF8";

/// Convenience macro to extract a `Handle<JsBox<Connection>>` from the first
/// argument passed to a JavaScript function implemented here.
///
#[macro_export]
macro_rules! connection_arg_0 {
  ( $x:expr ) => {
    $x.argument::<JsBox<crate::connection::Connection>>(0)?
  };
}

pub use connection_arg_0;

/// Status of a PostgreSQL connection.
///
/// As we establish connections in a _blocking_ fashion, the only two statuses
/// we'll ever see are the following:
///
/// * `CONNECTION_OK`
/// * `CONNECTION_BAD`
///
/// The following values should be returned _only_ when establishing connections
/// asynchronously:
///
/// * `CONNECTION_STARTED`: Waiting for connection to be made.
/// * `CONNECTION_MADE`: Connection OK; waiting to send.
/// * `CONNECTION_AWAITING_RESPONSE`: Waiting for a response from the server.
/// * `CONNECTION_AUTH_OK`: Received authentication; waiting for backend start-up to finish.
/// * `CONNECTION_SETENV`: Negotiating environment-driven parameter settings.
/// * `CONNECTION_SSL_STARTUP`: Negotiating SSL encryption.
/// * `CONNECTION_NEEDED`: _Internal state: `connect()` needed_.
/// * `CONNECTION_CHECK_WRITABLE`: Checking if connection is able to handle write transactions.
/// * `CONNECTION_CONSUME`: Consuming any remaining response messages on connection.
/// * `CONNECTION_GSS_STARTUP`: Negotiating GSSAPI.
/// * `CONNECTION_CHECK_TARGET`: _Internal state: checking target server properties_.
/// * `CONNECTION_CHECK_STANDBY`: Checking if server is in standby mode.
///
/// See [PQconnectStartParams](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTSTARTPARAMS)
///
pub type ConnStatusType = pq_sys::ConnStatusType;

// ===== DEFINITION ============================================================

/// Struct wrapping the LibPQ functions related to a _connection_.
///
/// Normally this is wrapped in a Neon `JsBox` and managed by NodeJS.
///
#[derive(Debug)]
pub struct Connection {
  connection: *mut pq_sys::pg_conn
}

// ===== TRAITS ================================================================

impl Drop for Connection {
  /// Closes the connection to the server. Also frees memory used by the PGconn object.
  ///
  /// See [`PQfinish`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQFINISH)
  ///
  fn drop(&mut self) {
    unsafe {
      if ! self.connection.is_null() {
        println!("Finishing connection");
        pq_sys::PQfinish(self.connection);
        self.connection = std::ptr::null_mut();
      }
    }
  }
}

impl Finalize for Connection {
  /// Calls the destructor when JavaScript garbage collects this.
  ///
  fn finalize<'a, C: Context<'a>>(self, _: &mut C) {
    println!("Finalizing connection");
    drop(self);
  }
}

unsafe impl Send for Connection {}

// ===== IMPL ==================================================================

impl Connection {
  /// Makes a new connection to the database server.
  ///
  /// See [`PQconnectdbParams`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTDBPARAMS)
  ///
  pub fn pq_connectdb_params(info: ConnInfo) -> Result<Self, String> {
    let mut keys = Vec::<&str>::from([ ENCODING_KEY ]);
    let mut values = Vec::<&str>::from([ ENCODING_VAL ]);

    for (key, value) in info.iter() {
      // strip client encoding, we only use UTF8
      if *key == ENCODING_KEY { continue }

      // push anything else
      keys.push(key.as_str());
      values.push(value.as_str());
    }

    let k = utils::NullTerminatedArray::new(&keys);
    let v = utils::NullTerminatedArray::new(&values);

    let connection = unsafe {
      let conn = pq_sys::PQconnectdbParams(k.as_vec().as_ptr(), v.as_vec().as_ptr(), 0);
      match conn.is_null() {
        true => Err("Unable to create connection"),
        _ => Ok(Connection { connection: conn }),
      }
    }?;

    match connection.pq_status() {
      ConnStatusType::CONNECTION_OK => Ok(connection),
      _ => {
        let message = connection.pq_error_message()
          .unwrap_or("Unknown error".to_string());
        Err(message)
      }
    }
  }

  /// Returns the connection options used by a live connection.
  ///
  /// See [`PQconninfo`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFO)
  ///
  pub fn pq_conninfo(&self) -> Result<ConnInfo, String> {
    unsafe {
      ConnInfo::from_raw(pq_sys::PQconninfo(self.connection))
    }
  }

  // ===== STATUS ==============================================================

  /// Returns the status of the connection.
  ///
  /// See [`PQstatus`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSTATUS)
  ///
  pub fn pq_status(&self) -> ConnStatusType {
    unsafe { pq_sys::PQstatus(self.connection) }
  }

  /// Returns the current in-transaction status of the server.
  ///
  /// See [`PQtransactionStatus`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQTRANSACTIONSTATUS)
  ///
  pub fn pq_transaction_status(&self) -> pq_sys::PGTransactionStatusType {
    unsafe { pq_sys::PQtransactionStatus(self.connection) }
  }

  /// Returns the server version as a `String`.
  ///
  /// See [`PQserverVersion`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSERVERVERSION)
  ///
  pub fn pq_server_version(&self) -> Option<String> {
    let version = unsafe { pq_sys::PQserverVersion(self.connection) };
    match version {
      0 => None,
      _ => {
        let major = version / 10000;
        let minor = version % 10000;
        Some(format!("{major}.{minor}"))
      },
    }
  }

  /// Returns the error message most recently generated by an operation on the connection.
  ///
  /// See [`PQerrorMessage`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQERRORMESSAGE)
  ///
  pub fn pq_error_message(&self) -> Option<String> {
    let message = unsafe { pq_sys::PQerrorMessage(self.connection) };
    match message.is_null() {
      true => None,
      false => {
        let msg = utils::to_string(message)
          .unwrap_or_else(|msg| format!("Unknown error: {}", msg));
        match msg.len() {
          0 => None,
          _ => Some(msg.trim().to_string()),
        }
      },
    }
  }

  /// Obtains the file descriptor number of the connection socket to the server.
  ///
  /// See [`PQsocket`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSOCKET)
  ///
  pub fn pq_socket(&self) -> i32 {
    unsafe { pq_sys::PQsocket(self.connection) }
  }

  /// Returns the process ID (PID) of the backend process handling this connection.
  ///
  /// See [`PQbackendPID`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQBACKENDPID)
  ///
  pub fn pq_backend_pid(&self) -> i32 {
    unsafe { pq_sys::PQbackendPID(self.connection) }
  }

  /// Returns `true` if the connection uses SSL, `false` if not.
  ///
  /// See [`PQsslInUse`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSSLINUSE)
  ///
  pub fn pq_ssl_in_use(&self) -> bool {
    unsafe { pq_sys::PQsslInUse(self.connection) != 0 }
  }

  /// Returns SSL-related information about the connection.
  ///
  /// This returns a `Vec` of `(String,String)` tuples containing _only_ the
  /// key-value mappings for non-null attributes.
  ///
  /// See [`PQsslAttribute`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSSLATTRIBUTE)
  /// See [`PQsslAttributeNames`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSSLATTRIBUTENAMES)
  ///
  pub fn pq_ssl_attributes(&self) -> Result<Vec<(String, String)>, String> {
    let mut strings = Vec::<(String, String)>::new();

    unsafe {
      let raw = pq_sys::PQsslAttributeNames(self.connection);

      for x in 0.. {
        if (*raw.offset(x)).is_null() {
          break;
        } else {
          let key_ptr = *raw.offset(x);
          let val_ptr = pq_sys::PQsslAttribute(self.connection, key_ptr);
          if val_ptr.is_null() {
            continue;
          }

          let key = utils::to_string(key_ptr)?;
          let val = utils::to_string(val_ptr)?;
          strings.push((key, val));
        }
      }
    };

    Ok(strings)
  }

  // ===== ASYNC ===============================================================

  /// If input is available from the server, consume it.
  ///
  /// See [`PQconsumeInput`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQCONSUMEINPUT)
  ///
  pub fn pq_consume_input(&self) -> Result<(), String> {
    let result = unsafe { pq_sys::PQconsumeInput(self.connection) };
    match result {
      1 => Ok(()),
      _ => Err(self.pq_error_message().unwrap_or("Unknown error".to_string())),
    }
  }

  /// Returns `true` if a command is busy, that is, `pq_get_result` would block
  /// waiting for input. A `false` return indicates that `pq_get_result` can be
  /// called with assurance of not blocking.
  ///
  /// See [`PQisBusy`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQISBUSY)
  ///
  pub fn pq_is_busy(&self) -> bool {
    unsafe { pq_sys::PQisBusy(self.connection) == 1 }
  }

  /// Sets the nonblocking status of the connection.
  ///
  /// See [`PQsetnonblocking`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSETNONBLOCKING)
  ///
  pub fn pq_setnonblocking(&self, nonblocking: bool) -> Result<(), String> {
    let arg = match nonblocking {
      false => 0,
      true => 1,
    };

    let result = unsafe { pq_sys::PQsetnonblocking(self.connection, arg) };

    match result {
      0 => Ok(()),
      _ => Err(self.pq_error_message().unwrap_or("Unknown error".to_string())),
    }
  }

  /// Returns the nonblocking status of the database connection.
  ///
  /// See [`PQisnonblocking`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQISNONBLOCKING)
  ///
  pub fn pq_isnonblocking(&self) -> bool {
    unsafe { pq_sys::PQisnonblocking(self.connection) == 1 }
  }

  /// Attempts to flush any queued output data to the server.
  ///
  /// Returns `true` if successful (or if the send queue is empty), an error
  /// if it failed for some reason, or `false` if it was unable to send all the
  /// data in the send queue yet (this case can only occur if the connection is
  /// nonblocking).
  ///
  /// See [`PQflush`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQFLUSH)
  ///
  pub fn pq_flush(&self) -> Result<bool, String> {
    let result = unsafe { pq_sys::PQflush(self.connection) };
    match result {
      0 => Ok(true), // data is all flushed
      1 => Ok(false), // still some data to flush
      _ => Err(self.pq_error_message().unwrap_or("Unknown error".to_string())),
    }
  }

  // ===== SYNCHRONOUS OPERATIONS ==============================================

  /// Submits a command to the server and waits for the result.
  ///
  /// See [`PQexec`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQEXEC)
  ///
  pub fn pq_exec(&self) { core::todo!() }

  /// Submits a command to the server and waits for the result, with the ability
  /// to pass parameters separately from the SQL command text.
  ///
  /// See [`PQexecParams`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQEXECPARAMS)
  ///
  pub fn pq_exec_params(&self) { core::todo!() }

  /// Submits a request to create a prepared statement with the given
  /// parameters, and waits for completion.
  ///
  /// See [`PQprepare`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQPREPARE)
  ///
  pub fn pq_prepare(&self) { core::todo!() }

  /// Sends a request to execute a prepared statement with given parameters,
  /// and waits for the result.
  ///
  /// See [`PQexecPrepared`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQEXECPREPARED)
  ///
  pub fn pq_exec_prepared(&self) { core::todo!() }

  /// Submits a request to obtain information about the specified prepared
  /// statement, and waits for completion.
  ///
  /// See [`PQdescribePrepared`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQDESCRIBEPREPARED)
  ///
  pub fn pq_describe_prepared(&self) { core::todo!() }

  /// Submits a request to obtain information about the specified portal, and
  /// waits for completion.
  ///
  /// See [`PQdescribePortal`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQDESCRIBEPORTAL)
  ///
  pub fn pq_describe_portal(&self) { core::todo!() }

  // ===== ASYNCHRONOUS OPERATIONS =============================================

  /// Submits a command to the server without waiting for the result(s).
  ///
  /// Asynchronous version of [`Connection::pq_exec`].
  ///
  /// See [`PQsendQuery`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDQUERY)
  ///
  pub fn pq_send_query(&self) { core::todo!() }

  /// Submits a command and separate parameters to the server without waiting
  /// for the result(s).
  ///
  /// Asynchronous version of [`Connection::pq_exec_params`].
  ///
  /// See [`PQsendQueryParams`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDQUERYPARAMS)
  ///
  pub fn pq_send_query_params(&self) { core::todo!() }

  /// Sends a request to create a prepared statement with the given parameters,
  /// without waiting for completion.
  ///
  /// Asynchronous version of [`Connection::pq_prepare`].
  ///
  /// See [`PQsendPrepare`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDPREPARE)
  ///
  pub fn pq_send_prepare(&self) { core::todo!() }

  /// Sends a request to execute a prepared statement with given parameters,
  /// without waiting for the result(s).
  ///
  /// Asynchronous version of [`Connection::pq_exec_prepared`].
  ///
  /// See [`PQsendQueryPrepared`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDQUERYPREPARED)
  ///
  pub fn pq_send_query_prepared(&self) { core::todo!() }

  /// Submits a request to obtain information about the specified prepared
  /// statement, without waiting for completion.
  ///
  /// Asynchronous version of [`Connection::pq_describe_prepared`].
  ///
  /// See [`PQsendDescribePrepared`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDDESCRIBEPREPARED)
  ///
  pub fn pq_send_describe_prepared(&self) { core::todo!() }

  /// Submits a request to obtain information about the specified portal, without waiting for completion.
  ///
  /// Asynchronous version of [`Connection::pq_describe_portal`].
  ///
  /// See [`PQsendDescribePortal`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDDESCRIBEPORTAL)
  ///
  pub fn pq_send_describe_portal(&self) { core::todo!() }

  /// Waits for the next result from a prior [`Connection::pq_send_query`],
  /// [`Connection::pq_send_query_params`],
  /// [`Connection::pq_send_prepare`],
  /// [`Connection::pq_send_query_prepared`],
  /// [`Connection::pq_send_describe_prepared`],
  /// [`Connection::pq_send_describe_portal`], or
  /// [`Connection::pq_pipeline_sync`] call, and returns it.
  ///
  /// See [`PQgetResult`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQGETRESULT)
  ///
  pub fn pq_get_result(&self) { core::todo!() }

  // ===== PIPELINE MODE =======================================================

  /// Returns the current pipeline mode status of the libpq connection.
  ///
  /// See [`PQpipelineStatus`](https://www.postgresql.org/docs/current/libpq-pipeline-mode.html#LIBPQ-PQPIPELINESTATUS)
  ///
  pub fn pq_pipeline_status(&self) { core::todo!() }

  /// Causes a connection to enter pipeline mode if it is currently idle or
  /// already in pipeline mode.
  ///
  /// See [`PQenterPipelineMode`](https://www.postgresql.org/docs/current/libpq-pipeline-mode.html#LIBPQ-PQENTERPIPELINEMODE)
  ///
  pub fn pq_enter_pipeline_mode(&self) { core::todo!() }

  /// Causes a connection to exit pipeline mode if it is currently in pipeline
  /// mode with an empty queue and no pending results.
  ///
  /// See [`PQexitPipelineMode`](https://www.postgresql.org/docs/current/libpq-pipeline-mode.html#LIBPQ-PQEXITPIPELINEMODE)
  ///
  pub fn pq_exit_pipeline_mode(&self) { core::todo!() }

  /// Marks a synchronization point in a pipeline by sending a sync message and
  /// flushing the send buffer.
  ///
  /// See [`PQpipelineSync`](https://www.postgresql.org/docs/current/libpq-pipeline-mode.html#LIBPQ-PQPIPELINESYNC)
  ///
  pub fn pq_pipeline_sync(&self) { core::todo!() }

  /// Sends a request for the server to flush its output buffer.
  ///
  /// See [`PQsendFlushRequest`](https://www.postgresql.org/docs/current/libpq-pipeline-mode.html#LIBPQ-PQSENDFLUSHREQUEST)
  ///
  pub fn pq_send_flush_request(&self) { core::todo!() }


  // ===== SINGLE ROW MODE =====================================================

  /// Select single-row mode for the currently-executing query.
  ///
  /// See [`PQsetSingleRowMode`](https://www.postgresql.org/docs/current/libpq-single-row-mode.html#LIBPQ-PQSETSINGLEROWMODE)
  ///
  pub fn pq_set_single_row_mode(&self) { core::todo!() }

}
