use core::fmt;
use std::fmt::Debug;

/* ===== PUBLIC: NapiResult ================================================= */

pub type NapiResult<T> = Result<T, NapiError>;

/* ===== PUBLIC: NapiError ================================================== */

#[derive(Debug)]
pub struct NapiError {
  message: String,
}

impl std::fmt::Display for NapiError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl From<String> for NapiError {
  fn from(message: String) -> Self {
    Self { message }
  }
}

impl From<&str> for NapiError {
  fn from(message: &str) -> Self {
    Self { message: message.to_string() }
  }
}

// impl NapiError {
//   // pub fn code(&self) -> Option<&'static str> {
//   //   match self.status {
//   //     None => None,
//   //     Some(status) => Some(match status {
//   //       napi::Status::napi_ok => "napi_ok",
//   //       napi::Status::napi_invalid_arg => "napi_invalid_arg",
//   //       napi::Status::napi_object_expected => "napi_object_expected",
//   //       napi::Status::napi_string_expected => "napi_string_expected",
//   //       napi::Status::napi_name_expected => "napi_name_expected",
//   //       napi::Status::napi_function_expected => "napi_function_expected",
//   //       napi::Status::napi_number_expected => "napi_number_expected",
//   //       napi::Status::napi_boolean_expected => "napi_boolean_expected",
//   //       napi::Status::napi_array_expected => "napi_array_expected",
//   //       napi::Status::napi_generic_failure => "napi_generic_failure",
//   //       napi::Status::napi_pending_exception => "napi_pending_exception",
//   //       napi::Status::napi_cancelled => "napi_cancelled",
//   //       napi::Status::napi_escape_called_twice => "napi_escape_called_twice",
//   //       napi::Status::napi_handle_scope_mismatch => "napi_handle_scope_mismatch",
//   //       napi::Status::napi_callback_scope_mismatch => "napi_callback_scope_mismatch",
//   //       napi::Status::napi_queue_full => "napi_queue_full",
//   //       napi::Status::napi_closing => "napi_closing",
//   //       napi::Status::napi_bigint_expected => "napi_bigint_expected",
//   //       napi::Status::napi_date_expected => "napi_date_expected",
//   //       napi::Status::napi_arraybuffer_expected => "napi_arraybuffer_expected",
//   //       napi::Status::napi_detachable_arraybuffer_expected => "napi_detachable_arraybuffer_expected",
//   //       napi::Status::napi_would_deadlock => "napi_would_deadlock",
//   //       napi::Status::napi_no_external_buffers_allowed => "napi_no_external_buffers_allowed",
//   //       #[allow(unreachable_patterns)]
//   //       _ => "napi_unknown",
//   //     })
//   //   }
//   }
// }
