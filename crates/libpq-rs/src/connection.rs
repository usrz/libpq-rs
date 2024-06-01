//! Wrap LibPQ's own `PGconn` struct.

use crate::conninfo::PQConninfo;
use crate::debug::*;
use crate::errors::*;
use crate::ffi::*;
use crate::notices::*;
use crate::notifications::PQNotification;
use crate::response::PQResponse;
use polling::Event;
use polling::Events;
use polling::Poller;
use std::fmt::Debug;
use std::os::fd::BorrowedFd;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;
use std::time::Duration;

/// Key for our `client_encoding` which must be always `UTF8`
static ENCODING_KEY: &str = "client_encoding";
/// Value for our `client_encoding` which must be always `UTF8`
static ENCODING_VAL: &str = "UTF8";

/* ========================================================================== *
 * ENUMS                                                                      *
 * ========================================================================== */

/// Status of a PostgreSQL connection.
///
/// As we establish connections in a _blocking_ fashion, the only two statuses
/// we'll ever see are `PQConnection_OK` and `PQConnection_BAD`
///
/// The all other values should be returned _only_ when establishing connections
/// asynchronously.
///
/// See [PQconnectStartParams](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTSTARTPARAMS)
///
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PQConnectionStatus {
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

impl From<pq_sys::ConnStatusType> for PQConnectionStatus {
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
pub enum PQTransactionStatus {
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

impl From<pq_sys::PGTransactionStatusType> for PQTransactionStatus {
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

/// Polling interest for [`PQConnection::poll`].
///
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PQPollingInterest {
  /// Wait until the connection is writable.
  Writable = 0,
  /// Wait until the connection is readable.
  Readable = 1,
}

/* ========================================================================== *
 * CONNECTION                                                                 *
 * ========================================================================== */

/// Struct wrapping the LibPQ functions related to a _connection_.
///
pub struct PQConnection {
  connection: *mut pq_sys::pg_conn,
  notice_processor: AtomicPtr<PQNoticeProcessorWrapper>,
}

// ===== TRAITS ================================================================

debug_self!(PQConnection, connection, "@");

impl Drop for PQConnection {
  /// Closes the connection to the server. Also frees memory used by the PGconn object.
  ///
  /// See [`PQfinish`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQFINISH)
  ///
  fn drop(&mut self) {
    debug_drop!(self);
    unsafe { pq_sys::PQfinish(self.connection) };
  }
}

impl TryFrom<&str> for PQConnection {
  type Error = PQError;

  /// Makes a new connection to the database server using a PostgreSQL
  /// connection string (DSN).
  ///
  /// See [`PQconnectdbParams`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTDBPARAMS)
  /// See [`PQconninfoParse`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFOPARSE)
  ///
  fn try_from(value: &str) -> PQResult<Self> {
    PQConnection::try_from(PQConninfo::try_from(value)?)
  }
}

impl TryFrom<String> for PQConnection {
  type Error = PQError;

  /// Makes a new connection to the database server using a PostgreSQL
  /// connection string (DSN).
  ///
  /// See [`PQconnectdbParams`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTDBPARAMS)
  /// See [`PQconninfoParse`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFOPARSE)
  ///
  fn try_from(value: String) -> PQResult<Self> {
    PQConnection::try_from(PQConninfo::try_from(value)?)
  }
}

impl TryFrom<PQConninfo> for PQConnection {
  type Error = PQError;

  /// Makes a new connection to the database server.
  ///
  /// See [`PQconnectdbParams`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTDBPARAMS)
  ///
  ///
  fn try_from(info: PQConninfo) -> PQResult<Self> {
    let mut keys = Vec::<&str>::from([ ENCODING_KEY ]);
    let mut values = Vec::<&str>::from([ ENCODING_VAL ]);

    for (key, value) in info.iter() {
      // strip client encoding, we only use UTF8
      if *key == ENCODING_KEY { continue }

      // push anything else
      keys.push(key.as_str());
      values.push(value.as_str());
    }

    let k = NullTerminatedArray::from(keys);
    let v = NullTerminatedArray::from(values);

    unsafe {
      pq_sys::PQconnectdbParams(
        k.as_vec().as_ptr(),
        v.as_vec().as_ptr(),
        0).try_into()
    }
  }
}

impl TryFrom<*mut pq_sys::pg_conn> for PQConnection {
  type Error = PQError;

  fn try_from(conn: *mut pq_sys::pg_conn) -> PQResult<Self> {
    let notice_processor = AtomicPtr::new(null_mut());

    let connection = match conn.is_null() {
      true => Err("Unable to create connection (null ptr)"),
      _ => Ok(PQConnection { connection: conn, notice_processor })
    }?;

    let connection = debug_create!(connection);

    let notice_processor = PQDefaultNoticeProcessor::new();
    connection.pq_set_notice_processor(Box::new(notice_processor));

    match connection.pq_status() {
      PQConnectionStatus::Ok => Ok(connection),
      _ => Err(PQError::from(&connection)),
    }
  }
}

unsafe impl Send for PQConnection {}
unsafe impl Sync for PQConnection {}

// ===== IMPL ==================================================================

impl PQConnection {

  // ===== CONNECTION ==========================================================

  /// Makes a new connection to the database server using LibPQ's own defaults..
  ///
  /// See [`PQconnectdbParams`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNECTDBPARAMS)
  /// See [`PQconndefaults`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNDEFAULTS)
  ///
  pub fn new() -> PQResult<Self> {
    PQConnection::try_from(PQConninfo::default())
  }

  /// Returns the connection options used by a live connection.
  ///
  /// See [`PQconninfo`](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PQCONNINFO)
  ///
  pub fn pq_conninfo(&self) -> PQResult<PQConninfo> {
    unsafe { PQConninfo::try_from(pq_sys::PQconninfo(self.connection)) }
  }

  /// Sets the current notice processor.
  ///
  /// See [PQnoticeProcessor](https://www.postgresql.org/docs/current/libpq-notice-processing.html)
  /// See [PQnoticeReceiverr](https://www.postgresql.org/docs/current/libpq-notice-processing.html)
  pub fn pq_set_notice_processor(&self, notice_processor: Box<dyn PQNoticeProcessor>) {
    #[cfg(debug_assertions)]
    let to_string = format!("{:?}", notice_processor);

    let wrapper = PQNoticeProcessorWrapper::from(notice_processor);

    let boxed = Box::new(wrapper);
    let pointer = Box::into_raw(boxed);
    let old_pointer = self.notice_processor.swap(pointer, Ordering::Relaxed);

    debug!("Setting up new notice processor at {:?}: {:?}", pointer, to_string);

    unsafe {
      pq_sys::PQsetNoticeReceiver(
        self.connection,
        Some(shared_notice_processor),
        pointer as *mut c_void,
      )
    };

    if old_pointer.is_null() {
      debug!("Not reclaiming old notice processor (null ptr)");
    } else {
      debug!("Reclaiming old notice processor at {:?}", old_pointer);
      drop(unsafe { Box::from_raw(old_pointer) });
    }
  }

  // ===== STATUS ==============================================================

  /// Returns the status of the connection.
  ///
  /// See [`PQstatus`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSTATUS)
  ///
  pub fn pq_status(&self) -> PQConnectionStatus {
    unsafe { pq_sys::PQstatus(self.connection).into() }
  }

  /// Returns the current in-transaction status of the server.
  ///
  /// See [`PQtransactionStatus`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQTRANSACTIONSTATUS)
  ///
  pub fn pq_transaction_status(&self) -> PQTransactionStatus {
    unsafe { pq_sys::PQtransactionStatus(self.connection).into() }
  }

  /// Returns the server version as a `String`.
  ///
  /// See [`PQserverVersion`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSERVERVERSION)
  ///
  pub fn pq_server_version(&self) -> Option<String> {
    unsafe {
      match pq_sys::PQserverVersion(self.connection) {
        0 => None,
        version => {
          let major = version / 10000;
          let minor = version % 10000;
          Some(format!("{major}.{minor}"))
        },
      }
    }
  }

  /// Returns the error message most recently generated by an operation on the connection.
  ///
  /// See [`PQerrorMessage`](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQERRORMESSAGE)
  ///
  pub fn pq_error_message(&self) -> Option<String> {
    unsafe {
      let message = pq_sys::PQerrorMessage(self.connection);
      match to_string_lossy(message) {
        Some(message) => Some(message.trim().to_string()),
        None => None,
      }
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
  pub fn pq_ssl_attributes(&self) -> PQResult<Vec<(String, String)>> {
    unsafe {
      let mut strings = Vec::<(String, String)>::new();
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

          let key = to_string_lossy(key_ptr);
          let val = to_string_lossy(val_ptr);
          if key.is_some() && val.is_some() {
            strings.push((key.unwrap(), val.unwrap()));
          }
        }
      }

      Ok(strings)
    }
  }

  // ===== ASYNC ===============================================================

  /// If input is available from the server, consume it.
  ///
  /// See [`PQconsumeInput`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQCONSUMEINPUT)
  ///
  pub fn pq_consume_input(&self) -> PQResult<()> {
    unsafe {
      match pq_sys::PQconsumeInput(self.connection) {
        1 => Ok(()),
        _ => Err(PQError::from(self)),
      }
    }
  }

  /// Returns `true` if a command is busy, that is [`PQConnection::pq_get_result`]
  /// would block waiting for input. A `false` return indicates that
  /// [`PQConnection::pq_get_result`] can be called with assurance of not blocking.
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
  pub fn pq_setnonblocking(&self, nonblocking: bool) -> PQResult<()> {
    unsafe {
      match pq_sys::PQsetnonblocking(self.connection, nonblocking as i32) {
        0 => Ok(()),
        _ => Err(PQError::from(self)),
      }
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
  pub fn pq_flush(&self) -> PQResult<bool> {
    unsafe {
      match pq_sys::PQflush(self.connection) {
        0 => Ok(true), // data is all flushed
        1 => Ok(false), // still some data to flush
        _ => Err(PQError::from(self)),
      }
    }
  }

  // ===== ASYNCHRONOUS OPERATIONS =============================================

  /// Submits a command to the server without waiting for the result(s).
  ///
  /// See [`PQsendQuery`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDQUERY)
  ///
  pub fn pq_send_query(&self, command: String) -> PQResult<()> {
    unsafe {
      let string = to_cstring(command.as_str());
      match pq_sys::PQsendQuery(self.connection, string.as_ptr()) {
        1 => Ok(()), // successful!
        _ => Err(PQError::from(self)),
      }
    }
  }

  /// Submits a command and separate parameters to the server without waiting
  /// for the result(s).
  ///
  /// See [`PQsendQueryParams`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQSENDQUERYPARAMS)
  ///
  pub fn pq_send_query_params(&self, command: String, params: Vec<String>) -> PQResult<()> {
    unsafe {
      let string = to_cstring(command.as_str());
      let arguments_length = params.len();
      let arguments = NullTerminatedArray::from(params);
      match pq_sys::PQsendQueryParams(
        self.connection,
        string.as_ptr(),
        arguments_length.try_into().unwrap(),
        std::ptr::null(),
        arguments.as_vec().as_ptr(),
        std::ptr::null(),
        std::ptr::null(),
        0, // always text!
      ) {
        1 => Ok(()), // successful!
        _ => Err(PQError::from(self)),
      }
    }
  }

  /// Waits for the next result from a prior [`PQConnection::pq_send_query`], or
  /// [`PQConnection::pq_send_query_params`], and returns it.
  ///
  /// See [`PQgetResult`](https://www.postgresql.org/docs/current/libpq-async.html#LIBPQ-PQGETRESULT)
  ///
  pub fn pq_get_result(&self) -> Option<PQResponse> {
    unsafe {
      let result = pq_sys::PQgetResult(self.connection);
      match result.is_null() {
        false => Some(PQResponse::try_from(result).unwrap()),
        true => None,
      }
    }
  }

  /// Returns a vector of all unhandled notifications received from the server.
  ///
  /// See [PQnotifies](https://www.postgresql.org/docs/current/libpq-notify.html)
  ///
  pub fn pq_notifies(&self) -> PQResult<Vec<PQNotification>> {
    let mut vec = Vec::<PQNotification>::new();

    unsafe {
      for _ in 0.. {
        // The "pgNotify" struct has a "next" pointer to it, but LibPQ's own
        // source explicitly mentions that it shouldn't be used in client code.
        let result = pq_sys::PQnotifies(self.connection);
        if result.is_null() {
          break;
        }

        vec.push(PQNotification::try_from(result)?);
        pq_sys::PQfreemem(result as *mut c_void);
      }
    }

    Ok(vec)
  }

  // ===== SINGLE ROW MODE =====================================================

  /// Select single-row mode for the currently-executing query.
  ///
  /// See [`PQsetSingleRowMode`](https://www.postgresql.org/docs/current/libpq-single-row-mode.html#LIBPQ-PQSETSINGLEROWMODE)
  ///
  pub fn pq_set_single_row_mode(&self) -> bool {
    unsafe { pq_sys::PQsetSingleRowMode(self.connection) == 1 }
  }

  // ===== POLLING =============================================================

  /// Wait until reads from or writes to the connection will not block.
  ///
  pub fn poll(&self, interest: PQPollingInterest, timeout: Option<Duration>) -> PQResult<()> {
    let key = debug_id();

    let event = match interest {
      PQPollingInterest::Readable => Event::readable(key),
      PQPollingInterest::Writable => Event::writable(key),
    };

    let poller = Poller::new()
      .map_err(| err | format!("Error creating poller: {}", err))?;

    let source = unsafe {
      let fd = pq_sys::PQsocket(self.connection);
      let source = BorrowedFd::borrow_raw(fd);
      poller.add(&source, event)
        .map_err(| err | format!("Error adding to poller: {}", err))?;
      source
    };

    let result = 'outer: loop {
      let mut events = Events::new();

      poller.wait(&mut events, timeout)
        .map_err(| err | format!("Error waiting on poller: {}", err))?;

      if events.is_empty() { println!("NOPE"); continue 'outer }

      'inner: for event in events.iter() {
        if event.key != key { continue 'inner; }
        if event.is_interrupt() { break 'outer Err(PQError::from("Connection interrupted")) }
        if event.is_err() == Some(true) { break 'outer Err(PQError::from("Connection error")) }

        match interest {
          PQPollingInterest::Readable => if event.readable { println!("YES R"); break 'outer Ok(()) },
          PQPollingInterest::Writable => if event.writable { println!("YES W"); break 'outer Ok(()) },
        }
      }
    };

    // Remember to _delete_ our source!
    poller.delete(source).map_err(|err| format!("Error deleting from poller: {}", err))?;

    // Return our result
    result
  }
}
