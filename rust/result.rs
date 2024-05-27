use crate::debug;
use crate::errors::*;

#[derive(Debug)]
pub struct ToDoResult {
  result: *mut pq_sys::pg_result,
}

impl Drop for ToDoResult {
  /// Frees the storage associated with a `PGresult`.
  ///
  /// See [`PQclear`](https://www.postgresql.org/docs/current/libpq-exec.html#LIBPQ-PQCLEAR)
  ///
  fn drop(&mut self) {
    debug!("Dropping result {:?}", self);
    unsafe { pq_sys::PQclear(self.result) };
  }
}

impl TryFrom<*mut pq_sys::pg_result> for ToDoResult {
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

impl ToDoResult {
  pub fn pq_result_status(&self) -> String {
    unsafe {
      format!("RESULT STATUS {:?}", pq_sys::PQresultStatus(self.result))
    }
  }
}
