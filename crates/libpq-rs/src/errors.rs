//! Errors stuff.

use crate::connection::PQConnection;

/// The root of all evil: any error thrown by LibPQ.
///
#[derive(Debug, Clone)]
pub struct PQError {
  pub message: String
}

impl From<Option<String>> for PQError {
  /// Create a [`PQError`] from an _optional_ [`String`].
  ///
  fn from(message: Option<String>) -> Self {
    match message {
      Some(message) => Self { message },
      None => Self::from("Unknown error".to_string()),
    }
  }
}

impl From<&PQConnection> for PQError {
  /// Create a [`PQError`] from a [`PQConnection`]'s own
  /// [error message][PQConnection::pq_error_message].
  ///
  fn from(value: &PQConnection) -> Self {
    Self::from(value.pq_error_message())
  }
}

impl From<String> for PQError {
  /// Create a [`PQError`] from a [`String`].
  ///
  fn from(message: String) -> Self {
      Self{ message }
  }
}

impl From<&str> for PQError {
  /// Create a [`PQError`] from a [`str`]_ing_.
  ///
  fn from(message: &str) -> Self {
      Self{ message: message.to_string() }
  }
}

impl std::fmt::Display for PQError {
  /// Standard way to display a [`PQError`].
  ///
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[LibPQ Error]: {}", self.message)
  }
}

impl std::error::Error for PQError {}

/* ========================================================================== */

/// The result type for all LibPQ errors.
///
pub type PQResult<T> = Result<T, PQError>;
