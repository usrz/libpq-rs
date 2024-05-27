//! Wrap LibPQ's own `pg_result` struct.

use crate::debug;
use crate::errors::*;
use std::fmt::Debug;
use std::any::type_name;
use crate::debug_self;
use crate::debug_drop;
use crate::debug_create;

/// The result status of the command.
///
/// See [`PQresultStatus`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQRESULTSTATUS)
///
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ResponseStatus {
  /// The string sent to the server was empty.
  EmptyQuery = 0,
  /// Successful completion of a command returning no data.
  CommandOk = 1,
  /// Successful completion of a command returning data.
  TuplesOk = 2,
  /// Copy Out (from server) data transfer started.
  CopyOut = 3,
  /// Copy In (to server) data transfer started.
  CopyIn = 4,
  /// The server's response was not understood.
  BadResponse = 5,
  /// A nonfatal error (a notice or warning) occurred.
  NonfatalError = 6,
  /// A fatal error occurred.
  FatalError = 7,
  /// Copy In/Out (to and from server) data transfer started.
  CopyBoth = 8,
  /// The PGresult contains a single result tuple from the current command.
  SingleTuple = 9,
  /// The PGresult represents a synchronization point in pipeline mode.
  PipelineSync = 10,
  /// The PGresult represents a pipeline that has received an error from the server.
  PipelineAborted = 11,
}

impl From<pq_sys::ExecStatusType> for ResponseStatus {
  fn from(status: pq_sys::ExecStatusType) -> Self {
    match status {
      pq_sys::ExecStatusType::PGRES_EMPTY_QUERY => Self::EmptyQuery,
      pq_sys::ExecStatusType::PGRES_COMMAND_OK => Self::CommandOk,
      pq_sys::ExecStatusType::PGRES_TUPLES_OK => Self::TuplesOk,
      pq_sys::ExecStatusType::PGRES_COPY_OUT => Self::CopyOut,
      pq_sys::ExecStatusType::PGRES_COPY_IN => Self::CopyIn,
      pq_sys::ExecStatusType::PGRES_BAD_RESPONSE => Self::BadResponse,
      pq_sys::ExecStatusType::PGRES_NONFATAL_ERROR => Self::NonfatalError,
      pq_sys::ExecStatusType::PGRES_FATAL_ERROR => Self::FatalError,
      pq_sys::ExecStatusType::PGRES_COPY_BOTH => Self::CopyBoth,
      pq_sys::ExecStatusType::PGRES_SINGLE_TUPLE => Self::SingleTuple,
      pq_sys::ExecStatusType::PGRES_PIPELINE_SYNC => Self::PipelineSync,
      pq_sys::ExecStatusType::PGRES_PIPELINE_ABORTED => Self::PipelineAborted,
    }
  }
}

/* ========================================================================== */

/// Struct wrapping the LibPQ functions related to a _result_.
///
/// In order to avoid naming conflict with Rust's well known [`Result`], the
/// name of this struct is `PQResponse`.
///
pub struct PQResponse {
  result: *mut pq_sys::pg_result,
}

debug_self!(PQResponse, result, "@");

impl Drop for PQResponse {
  /// Frees the storage associated with a `PGresult` structure.
  ///
  /// See [`PQclear`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQCLEAR)
  ///
  fn drop(&mut self) {
    debug_drop!(self);
    unsafe { pq_sys::PQclear(self.result) };
  }
}

impl TryFrom<*mut pq_sys::pg_result> for PQResponse {
  type Error = PQError;

  /// Create a [`PQResponse`] from a LibPQ own `PGresult` structure.
  ///
  fn try_from(result: *mut pq_sys::pg_result) -> PQResult<Self> {
    match result.is_null() {
      true => Err("Unable to create connection".into()),
      _ => Ok(debug_create!(Self { result })),
    }
  }
}

impl PQResponse {
  pub fn pq_result_status(&self) -> ResponseStatus {
    unsafe {
      ResponseStatus::from(pq_sys::PQresultStatus(self.result))
    }
  }
}
