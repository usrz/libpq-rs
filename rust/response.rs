//! Wrap LibPQ's own `pg_result` struct.

use crate::debug;
use crate::errors::*;
use neon::prelude::*;
use std::fmt::Debug;
use std::any::type_name;
use crate::debug_self;
use crate::debug_drop;
use crate::debug_create;
use crate::ffi::to_string;
use crate::ffi::to_string_lossy;

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
  /// Returns the result status of the command.
  ///
  /// See [`PQresultStatus`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQRESULTSTATUS)
  ///
  pub fn pq_result_status(&self) -> ResponseStatus {
    unsafe {
      ResponseStatus::from(pq_sys::PQresultStatus(self.result))
    }
  }

  /// Returns the error message associated with the command, if any.
  ///
  /// See [`PQresultErrorMessage`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQRESULTERRORMESSAGE)
  ///
  pub fn pq_result_error_essage(&self) -> Option<String> {
    let message = unsafe {
      to_string_lossy(pq_sys::PQresultErrorMessage(self.result))
    };

    match message {
      None => None,
      Some(string) => {
        match string.trim() {
          "" => None,
          str => Some(str.to_string()),
        }
      }
    }
  }

  /// Returns the command status tag from the SQL command that generated the PGresult.
  ///
  /// See [`PQcmdStatus`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQCMDSTATUS)
  ///
  pub fn pq_cmd_status(&self) -> String {
    unsafe {
      to_string_lossy(pq_sys::PQcmdStatus(self.result))
        .unwrap_or("".to_string())
    }
  }

  /// Returns the number of rows affected by the SQL command.
  ///
  /// See [`PQcmdTuples`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQCMDTUPLES)
  ///
  pub fn pq_cmd_tuples(&self) -> i32 {
    unsafe {
      to_string_lossy(pq_sys::PQcmdTuples(self.result))
        .unwrap_or("".to_string())
        .parse::<i32>()
        .unwrap_or(0)
    }
  }

  /// Returns the number of rows (tuples) in the query result.
  ///
  /// See [`PQntuples`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQNTUPLES)
  ///
  pub fn pq_ntuples(&self) -> i32 {
    unsafe {
      pq_sys::PQntuples(self.result)
    }
  }

  /// Returns the number of columns (fields) in each row of the query result.
  ///
  /// See [`PQnfields`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQNFIELDS)
  ///
  pub fn pq_nfields(&self) -> i32 {
    unsafe {
      pq_sys::PQnfields(self.result)
    }
  }

  /// Returns the column name associated with the given column number.
  ///
  /// See [`PQfname`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFNAME)
  ///
  pub fn pq_fname(&self, column: i32) -> Option<String> {
    unsafe {
      to_string_lossy(pq_sys::PQfname(self.result, column))
    }
  }

  /// Returns the data type (the internal OID number) associated with the given
  /// column number.
  ///
  /// See [`PQftype`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQFTYPE)
  pub fn pq_ftype(&self, column: i32) -> u32 {
    unsafe {
      pq_sys::PQftype(self.result, column)
    }
  }

  /// Tests a field for a null value.
  ///
  /// See [`PQgetisnull`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQGETISNULL)
  ///
  pub fn pq_getisnull(&self, row: i32, column: i32) -> bool {
    unsafe {
      pq_sys::PQgetisnull(self.result, row, column) == 1
    }
  }

  /// Returns a single field value of one row of a PGresult.
  ///
  /// See [`PQgetvalue`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQGETVALUE)
  pub fn pq_getvalue(&self, row: i32, column: i32) -> PQResult<Option<String>> {
    match self.pq_getisnull(row, column) {
      true => Ok(None),
      false => unsafe {
        let ptr = pq_sys::PQgetvalue(self.result, row, column);
        // This is a *result*... it must be NON LOSSY!
        to_string(ptr).and_then(|string| Ok(Some(string)))
      }
    }
  }

  /// Converts the _contents_ (rows, columns, ...) of a [`PQResponse`] struct
  /// into a JavaScript object.
  ///
  pub fn to_js_object<'a, C: Context<'a>>(
    &self,
    cx: &mut C,
  ) -> NeonResult<Handle<'a, JsObject>> {
    let object = cx.empty_object();

    let status = self.pq_result_status();
    let status = cx.string(format!("{:?}", status));
    object.set(cx, "status", status)?;

    let command = cx.string(self.pq_cmd_status());
    let row_count = cx.number(self.pq_cmd_tuples());

    object.set(cx, "command", command)?;
    object.set(cx, "rowCount", row_count)?;

    let ntuples = self.pq_ntuples(); // rows
    let nfields = self.pq_nfields(); // columns

    debug!("Received {} rows and {} columns", ntuples, nfields);

    let fields = cx.empty_array();
    object.set(cx, "fields", fields)?;

    for i in 0..nfields {
      let tuple = cx.empty_array();
      let fname = cx.string(self.pq_fname(i).unwrap());
      let ftype = cx.number(self.pq_ftype(i));
      tuple.set(cx, 0, fname)?;
      tuple.set(cx, 1, ftype)?;
      fields.set(cx, i as u32, tuple)?;
    }


    let rows = cx.empty_array();
    object.set(cx, "rows", rows)?;

    for r in 0..ntuples {
      let row = cx.empty_array();
      rows.set(cx, r as u32, row)?;

      for c in 0..nfields {
        let value = self.pq_getvalue(r, c).or_throw(cx)?;
        if let Some(string) = value {
          let string = cx.string(string);
          row.set(cx, c as u32, string)?;
        }
      }
    }

    Ok(object)
  }
}
