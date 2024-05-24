//! Wrap the LibPQ function in a slightly more convenient _struct_.

use crate::conninfo::Conninfo;
use crate::sys::*;
use polling::Event;
use polling::Events;
use polling::Poller;
use pq_sys::pg_conn;
use std::os::fd::BorrowedFd;
use std::sync::RwLock;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Duration;
use self::utils::NullTerminatedArray;

static ENCODING_KEY: &str = "client_encoding";
static ENCODING_VAL: &str = "UTF8";
static POLLER_KEY: AtomicUsize = AtomicUsize::new(1);


// ===== ENUMS =================================================================

/// Status of a PostgreSQL connection.
///
/// As we establish connections in a _blocking_ fashion, the only two statuses
/// we'll ever see are `CONNECTION_OK` and `CONNECTION_BAD`
///
/// The all other values should be returned _only_ when establishing connections
/// asynchronously.
///
/// See [PQconnectStartParams](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTSTARTPARAMS)
///
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ConnectionStatus {
  /// Connection succesful.
  Ok = 0,
  /// Connection failed.
  Bad = 1,
  /// Waiting for connection to be made.
  Started = 2,
  /// Connection OK; waiting to send.
  Made = 3,
  /// Waiting for a response from the server.
  AwaitingResponse = 4,
  /// Received authentication; waiting for backend start-up to finish.
  AuthOk = 5,
  /// Negotiating environment-driven parameter settings.
  Setenv = 6,
  /// Negotiating SSL encryption.
  SslStartup = 7,
  /// **Internal state**: `connect()` needed.
  Needed = 8,
  /// Checking if connection is able to handle write transactions.
  CheckWritable = 9,
  /// Consuming any remaining response messages on connection.
  Consume = 10,
  /// Negotiating GSSAPI.
  GssStartup = 11,
  /// _Internal state: checking target server properties_.
  CheckTarget = 12,
  /// Checking if server is in standby mode.
  CheckStandby = 13,
}

impl From<pq_sys::ConnStatusType> for ConnectionStatus {
  fn from(status: pq_sys::ConnStatusType) -> Self {
    match status {
      pq_sys::ConnStatusType::CONNECTION_OK => Self::Ok,
      pq_sys::ConnStatusType::CONNECTION_BAD => Self::Bad,
      pq_sys::ConnStatusType::CONNECTION_STARTED => Self::Started,
      pq_sys::ConnStatusType::CONNECTION_MADE => Self::Made,
      pq_sys::ConnStatusType::CONNECTION_AWAITING_RESPONSE => Self::AwaitingResponse,
      pq_sys::ConnStatusType::CONNECTION_AUTH_OK => Self::AuthOk,
      pq_sys::ConnStatusType::CONNECTION_SETENV => Self::Setenv,
      pq_sys::ConnStatusType::CONNECTION_SSL_STARTUP => Self::SslStartup,
      pq_sys::ConnStatusType::CONNECTION_NEEDED => Self::Needed,
      pq_sys::ConnStatusType::CONNECTION_CHECK_WRITABLE => Self::CheckWritable,
      pq_sys::ConnStatusType::CONNECTION_CONSUME => Self::Consume,
      pq_sys::ConnStatusType::CONNECTION_GSS_STARTUP => Self::GssStartup,
      pq_sys::ConnStatusType::CONNECTION_CHECK_TARGET => Self::CheckTarget,
      pq_sys::ConnStatusType::CONNECTION_CHECK_STANDBY => Self::CheckStandby,
    }
  }
}

/// The current in-transaction status of the server.
///
/// See [`PQtransactionStatus`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQTRANSACTIONSTATUS)
///
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TransactionStatus {
  /// Currently idle.
  Idle = 0,
  /// A command is in progress.
  Active = 1,
  /// Idle, in a valid transaction block.
  InTransaction = 2,
  /// Idle, in a failed transaction block
  InError = 3,
  /// Reported if the connection is bad.
  Unknown = 4,
}

impl From<pq_sys::PGTransactionStatusType> for TransactionStatus {
  fn from(status: pq_sys::PGTransactionStatusType) -> Self {
    match status {
      pq_sys::PGTransactionStatusType::PQTRANS_IDLE => Self::Idle,
      pq_sys::PGTransactionStatusType::PQTRANS_ACTIVE => Self::Active,
      pq_sys::PGTransactionStatusType::PQTRANS_INTRANS => Self::InTransaction,
      pq_sys::PGTransactionStatusType::PQTRANS_INERROR => Self::InError,
      pq_sys::PGTransactionStatusType::PQTRANS_UNKNOWN => Self::Unknown,
    }
  }
}

/// The current pipeline mode status of the libpq connection.
///
/// See [`PQpipelineStatus`](https://www.postgresql.org/docs/current/libpq-pipeline-mode.html#LIBPQ-PQPIPELINESTATUS)
///
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PipelineStatus {
  /// The connection is _not_ in pipeline mode.
  Off = 0,
  /// The connection is in pipeline mode.
  On = 1,
  /// The libpq connection is in pipeline mode and an error occurred while
  /// processing the current pipeline.
  Aborted = 2,
}

impl From<pq_sys::PGpipelineStatus> for PipelineStatus {
  fn from(status: pq_sys::PGpipelineStatus) -> Self {
    match status {
      pq_sys::PGpipelineStatus::PQ_PIPELINE_OFF => Self::Off,
      pq_sys::PGpipelineStatus::PQ_PIPELINE_ON => Self::On,
      pq_sys::PGpipelineStatus::PQ_PIPELINE_ABORTED => Self::Aborted,
    }
  }
}

/// Polling interest for [`Connection::poll`].
///
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PollingInterest {
  /// Wait until the connection is writable.
  Writable = 0,
  /// Wait until the connection is readable.
  Readable = 1,
}

// ===== DEFINITION ============================================================

/// Struct wrapping the LibPQ functions related to a _connection_.
///
/// Normally this is wrapped in a Neon `JsBox` and managed by NodeJS.
///
pub struct Connection {
  connection: RwLock<*mut pq_sys::pg_conn>,
}

// ===== TRAITS ================================================================

impl Drop for Connection {
  /// Closes the connection to the server. Also frees memory used by the PGconn object.
  ///
  /// See [`PQfinish`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQFINISH)
  ///
  fn drop(&mut self) {
    let mut connection = self.connection.write().unwrap();

    if ! connection.is_null() {
      println!("Dropping connection");
      unsafe { pq_sys::PQfinish(*connection) };
      *connection = std::ptr::null_mut();
    } else {
      println!("Connection already dropped");
    }
  }
}

impl TryFrom<&str> for Connection {
  type Error = String;

  /// Makes a new connection to the database server using a PostgreSQL
  /// connection string (DSN).
  ///
  /// See [`PQconnectdbParams`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTDBPARAMS)
  /// See [`PQconninfoParse`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFOPARSE)
  ///
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Connection::try_from(Conninfo::try_from(value)?)
  }
}

impl TryFrom<String> for Connection {
  type Error = String;

  /// Makes a new connection to the database server using a PostgreSQL
  /// connection string (DSN).
  ///
  /// See [`PQconnectdbParams`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTDBPARAMS)
  /// See [`PQconninfoParse`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFOPARSE)
  ///
  fn try_from(value: String) -> Result<Self, Self::Error> {
    Connection::try_from(Conninfo::try_from(value)?)
  }
}

impl TryFrom<Conninfo> for Connection {
  type Error = String;

  /// Makes a new connection to the database server.
  ///
  /// See [`PQconnectdbParams`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTDBPARAMS)
  ///
  fn try_from(info: Conninfo) -> Result<Self, String> {
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
        _ => Ok(Connection { connection: RwLock::new(conn) })
        // TODO: notice processor!!!
      }
    }?;

    match connection.pq_status() {
      ConnectionStatus::Ok => Ok(connection),
      _ => {
        let message = connection.pq_error_message()
        .unwrap_or("Unknown error".to_string());
        Err(message)
      }
    }
  }
}

unsafe impl Send for Connection {}
unsafe impl Sync for Connection {}

// ===== IMPL ==================================================================

impl Connection {
  /// Execute closure read-locked on connection
  ///
  #[inline(always)]
  fn with_connection<O, E>(&self, execute: E) -> O
  where
    E: FnOnce(*mut pg_conn) -> O
  {
    let result = self.connection.read();
    match result {
      Err(_) => panic!("Unable to access connection"),
      Ok(guard) => {
        if guard.is_null() { panic!("Connection closed") };
        execute(*guard)
      }
    }
  }

  // ===== CONNECTION ==========================================================

  /// Makes a new connection to the database server using LibPQ's own defaults..
  ///
  /// See [`PQconnectdbParams`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTDBPARAMS)
  /// See [`PQconndefaults`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNDEFAULTS)
  ///
  pub fn new() -> Result<Self, String> {
    let info = Conninfo::new()?;
    Connection::try_from(info)
  }

  /// Returns the connection options used by a live connection.
  ///
  /// See [`PQconninfo`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFO)
  ///
  pub fn pq_conninfo(&self) -> Result<Conninfo, String> {
    self.with_connection(|connection| unsafe {
      Conninfo::try_from(pq_sys::PQconninfo(connection))
    })
  }

  // ===== STATUS ==============================================================

  /// Returns the status of the connection.
  ///
  /// See [`PQstatus`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSTATUS)
  ///
  pub fn pq_status(&self) -> ConnectionStatus {
    self.with_connection(|connection| unsafe {
      pq_sys::PQstatus(connection).into()
    })
  }

  /// Returns the current in-transaction status of the server.
  ///
  /// See [`PQtransactionStatus`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQTRANSACTIONSTATUS)
  ///
  pub fn pq_transaction_status(&self) -> TransactionStatus {
    self.with_connection(|connection| unsafe {
      pq_sys::PQtransactionStatus(connection).into()
    })
  }

  /// Returns the server version as a `String`.
  ///
  /// See [`PQserverVersion`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSERVERVERSION)
  ///
  pub fn pq_server_version(&self) -> Option<String> {
    self.with_connection(|connection| unsafe {
      match pq_sys::PQserverVersion(connection) {
        0 => None,
        version => {
          let major = version / 10000;
          let minor = version % 10000;
          Some(format!("{major}.{minor}"))
        },
      }
    })
  }

  /// Returns the error message most recently generated by an operation on the connection.
  ///
  /// See [`PQerrorMessage`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQERRORMESSAGE)
  ///
  pub fn pq_error_message(&self) -> Option<String> {
    self.with_connection(|connection| unsafe {
      let message = pq_sys::PQerrorMessage(connection);

      if message.is_null() { return None }

      let msg = utils::to_str(message)
        .unwrap_or("Unable to decode error message");

      if msg.is_empty() { return None }
      Some(msg.trim().to_string())
    })
  }

  /// Obtains the file descriptor number of the connection socket to the server.
  ///
  /// See [`PQsocket`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSOCKET)
  ///
  pub fn pq_socket(&self) -> i32 {
    self.with_connection(|connection| unsafe {
      pq_sys::PQsocket(connection)
    })
  }

  /// Returns the process ID (PID) of the backend process handling this connection.
  ///
  /// See [`PQbackendPID`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQBACKENDPID)
  ///
  pub fn pq_backend_pid(&self) -> i32 {
    self.with_connection(|connection| unsafe {
      pq_sys::PQbackendPID(connection)
    })
  }

  /// Returns `true` if the connection uses SSL, `false` if not.
  ///
  /// See [`PQsslInUse`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSSLINUSE)
  ///
  pub fn pq_ssl_in_use(&self) -> bool {
    self.with_connection(|connection| unsafe {
      pq_sys::PQsslInUse(connection) != 0
    })
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
    self.with_connection(|connection| unsafe {
      let mut strings = Vec::<(String, String)>::new();
      let raw = pq_sys::PQsslAttributeNames(connection);

      for x in 0.. {
        if (*raw.offset(x)).is_null() {
          break;
        } else {
          let key_ptr = *raw.offset(x);
          let val_ptr = pq_sys::PQsslAttribute(connection, key_ptr);
          if val_ptr.is_null() {
            continue;
          }

          let key = utils::to_string(key_ptr)?;
          let val = utils::to_string(val_ptr)?;
          strings.push((key, val));
        }
      }

      Ok(strings)
    })
  }

  // ===== ASYNC ===============================================================

  /// If input is available from the server, consume it.
  ///
  /// See [`PQconsumeInput`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQCONSUMEINPUT)
  ///
  pub fn pq_consume_input(&self) -> Result<(), String> {
    self.with_connection(|connection| unsafe {
      match pq_sys::PQconsumeInput(connection) {
        1 => Ok(()),
        _ => Err(self.pq_error_message().unwrap_or("Unknown error".to_string())),
      }
    })
  }

  /// Returns `true` if a command is busy, that is, `pq_get_result` would block
  /// waiting for input. A `false` return indicates that `pq_get_result` can be
  /// called with assurance of not blocking.
  ///
  /// See [`PQisBusy`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQISBUSY)
  ///
  pub fn pq_is_busy(&self) -> bool {
    self.with_connection(|connection| unsafe {
      pq_sys::PQisBusy(connection) == 1
    })
  }

  /// Sets the nonblocking status of the connection.
  ///
  /// See [`PQsetnonblocking`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSETNONBLOCKING)
  ///
  pub fn pq_setnonblocking(&self, nonblocking: bool) -> Result<(), String> {
    self.with_connection(|connection| unsafe {
      match pq_sys::PQsetnonblocking(connection, nonblocking as i32) {
        0 => Ok(()),
        _ => Err(self.pq_error_message().unwrap_or("Unknown error".to_string())),
      }
    })
  }

  /// Returns the nonblocking status of the database connection.
  ///
  /// See [`PQisnonblocking`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQISNONBLOCKING)
  ///
  pub fn pq_isnonblocking(&self) -> bool {
    self.with_connection(|connection| unsafe {
      pq_sys::PQisnonblocking(connection) == 1
    })
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
    self.with_connection(|connection| unsafe {
      match pq_sys::PQflush(connection) {
        0 => Ok(true), // data is all flushed
        1 => Ok(false), // still some data to flush
        _ => Err(self.pq_error_message().unwrap_or("Unknown error".to_string())),
      }
    })
  }

  // ===== ASYNCHRONOUS OPERATIONS =============================================

  /// Submits a command to the server without waiting for the result(s).
  ///
  /// Asynchronous version of [`Connection::pq_exec`].
  ///
  /// See [`PQsendQuery`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDQUERY)
  ///
  pub fn pq_send_query(&self, command: String) -> Result<(), String> {
    self.with_connection(|connection| unsafe {
      let string = utils::to_cstring(command.as_str());
      match pq_sys::PQsendQuery(connection, string.as_ptr()) {
        1 => Ok(()), // successful!
        _ => Err(self.pq_error_message().unwrap_or("Unknown error".to_string())),
      }
    })
  }

  /// Submits a command and separate parameters to the server without waiting
  /// for the result(s).
  ///
  /// Asynchronous version of [`Connection::pq_exec_params`].
  ///
  /// See [`PQsendQueryParams`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDQUERYPARAMS)
  ///
  pub fn pq_send_query_params(&self, command: String, params: Vec<String>) -> Result<(), String> {
    self.with_connection(|connection| unsafe {
      let string = utils::to_cstring(command.as_str());
      let arguments = NullTerminatedArray::new(&params);
      match pq_sys::PQsendQueryParams(
        connection,
        string.as_ptr(),
        params.len().try_into().unwrap(),
        std::ptr::null(),
        arguments.as_vec().as_ptr(),
        std::ptr::null(),
        std::ptr::null(),
        0, // always text!
      ) {
        1 => Ok(()), // successful!
        _ => Err(self.pq_error_message().unwrap_or("Unknown error".to_string())),
      }
    })
  }

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
  pub fn pq_get_result(&self) -> Result<String, String> {
    self.with_connection(|connection| unsafe {
      let result = pq_sys::PQgetResult(connection);

      match result.is_null() {
        true => Ok("DONE".to_string()),
        false => {
          pq_sys::PQclear(result);
          Ok(format!("RESULT STATUS {:?}", pq_sys::PQresultStatus(result)))
        }
      }
    })
  }

  // ===== PIPELINE MODE =======================================================

  /// Returns the current pipeline mode status of the libpq connection.
  ///
  /// See [`PQpipelineStatus`](https://www.postgresql.org/docs/current/libpq-pipeline-mode.html#LIBPQ-PQPIPELINESTATUS)
  ///
  pub fn pq_pipeline_status(&self) -> PipelineStatus {
    self.with_connection(|connection| unsafe {
      pq_sys::PQpipelineStatus(connection).into()
    })
  }

  /// Causes a connection to enter pipeline mode if it is currently idle or
  /// already in pipeline mode.
  ///
  /// See [`PQenterPipelineMode`](https://www.postgresql.org/docs/current/libpq-pipeline-mode.html#LIBPQ-PQENTERPIPELINEMODE)
  ///
  pub fn pq_enter_pipeline_mode(&self) -> bool {
    self.with_connection(|connection| unsafe {
      pq_sys::PQenterPipelineMode(connection) == 1
    })
  }

  /// Causes a connection to exit pipeline mode if it is currently in pipeline
  /// mode with an empty queue and no pending results.
  ///
  /// See [`PQexitPipelineMode`](https://www.postgresql.org/docs/current/libpq-pipeline-mode.html#LIBPQ-PQEXITPIPELINEMODE)
  ///
  pub fn pq_exit_pipeline_mode(&self) -> bool {
    self.with_connection(|connection| unsafe {
      pq_sys::PQexitPipelineMode(connection) == 1
    })
  }

  /// Marks a synchronization point in a pipeline by sending a sync message and
  /// flushing the send buffer.
  ///
  /// See [`PQpipelineSync`](https://www.postgresql.org/docs/current/libpq-pipeline-mode.html#LIBPQ-PQPIPELINESYNC)
  ///
  pub fn pq_pipeline_sync(&self) -> bool {
    self.with_connection(|connection| unsafe {
      pq_sys::PQpipelineSync(connection) == 1
    })
  }

  /// Sends a request for the server to flush its output buffer.
  ///
  /// See [`PQsendFlushRequest`](https://www.postgresql.org/docs/current/libpq-pipeline-mode.html#LIBPQ-PQSENDFLUSHREQUEST)
  ///
  pub fn pq_send_flush_request(&self) -> bool {
    self.with_connection(|connection| unsafe {
      pq_sys::PQsendFlushRequest(connection) == 1
    })
  }

  // ===== SINGLE ROW MODE =====================================================

  /// Select single-row mode for the currently-executing query.
  ///
  /// See [`PQsetSingleRowMode`](https://www.postgresql.org/docs/current/libpq-single-row-mode.html#LIBPQ-PQSETSINGLEROWMODE)
  ///
  pub fn pq_set_single_row_mode(&self) -> bool {
    self.with_connection(|connection| unsafe {
      pq_sys::PQsetSingleRowMode(connection) == 1
    })
  }

  // ===== POLLING =============================================================

  /// Wait until reads from or writes to the connection will not block.
  ///
  pub fn poll(&self, interest: PollingInterest, timeout: Option<Duration>) -> Result<(), String> {
    self.with_connection(|connection| {
      let fd = unsafe { pq_sys::PQsocket(connection) };

      let key = POLLER_KEY.fetch_add(1, Ordering::Relaxed);

      let event = match interest {
        PollingInterest::Readable => Event::readable(key),
        PollingInterest::Writable => Event::writable(key),
      };

      let poller = Poller::new()
        .or_else(| err | Err(format!("Error creating poller: {}", err)))?;

      let source = unsafe {
        let source = BorrowedFd::borrow_raw(fd);
        poller.add(&source, event)
          .or_else(| err | Err(format!("Error adding to poller: {}", err)))?;
        source
      };

      let mut events = Events::new();
      poller.wait(&mut events, timeout)
        .or_else(| err | Err(format!("Error waiting on poller: {}", err)))?;

      poller.delete(&source)
        .or_else(| err | Err(format!("Error deleting poller: {}", err)))?;

      Ok(())
    })
  }
}
