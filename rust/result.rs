//! Wrap LibPQ's own `pg_result` struct.

use crate::debug;
use crate::errors::*;

/// Struct wrapping the LibPQ functions related to a _result_.
///
/// In order to avoid naming conflict with Rust's well known [`Result`], the
/// name of this struct is `PQResponse`.
///
#[derive(Debug)]
pub struct PQResponse {
  result: *mut pq_sys::pg_result,
}

impl Drop for PQResponse {
  /// Frees the storage associated with a `PGresult`.
  ///
  /// See [`PQclear`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQCLEAR)
  ///
  fn drop(&mut self) {
    debug!("Dropping result {:?}", self);
    unsafe { pq_sys::PQclear(self.result) };
  }
}

impl TryFrom<*mut pq_sys::pg_result> for PQResponse {
  type Error = PQError;

  /// Create a [`PGResult`] from a LibPQ own `PGresult` structure.
  ///
  fn try_from(result: *mut pq_sys::pg_result) -> PQResult<Self> {
    match result.is_null() {
      true => Err("Unable to create connection".into()),
      _ => {
        let result = Self { result };
        debug!("Created result {:?}", result);
        Ok(result)
      }
    }
  }
}

impl PQResponse {
  pub fn pq_result_status(&self) -> String {
    unsafe {
      format!("RESULT STATUS {:?}", pq_sys::PQresultStatus(self.result))
    }
  }
}
