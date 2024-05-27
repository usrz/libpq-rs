use neon::prelude::*;

pub mod conn;
pub mod connection;
pub mod conninfo;
pub mod errors;
pub mod ffi;
pub mod notices;

/* ========================================================================== */

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
  ($($arg:tt)*) => {{ println!(">>> LIBPQ DEBUG >>> {}", format!($($arg)*)) }}
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
  ($($arg:tt)*) => {{}}
}

/* ========================================================================== */

/// Return the LibPQ version as a `String`
///
/// See [PQlibVersion](https://www.postgresql.org/docs/current/libpq-misc.html#LIBPQ-PQLIBVERSION)
///
fn libpq_version() -> String {
  let version = unsafe { pq_sys::PQlibVersion() };
  let major = version / 10000;
  let minor = version % 10000;
  format!("{major}.{minor}")
}

/// Return the OpenSSL version as a `String`
///
/// See [OPENSSL_VERSION_NUMBER](https://github.com/openssl/openssl/blob/master/include/openssl/opensslv.h.in#L92)
///
fn openssl_version() -> String {
  let version = unsafe { openssl_sys::OpenSSL_version_num() };
  let major = (version >> 28) & 0xF;
  let minor = (version >> 20) & 0xFF;
  let patch = (version >> 4) & 0xFFFF; // higher bite should be zero...
  let pre = version & 0xF;

  match pre {
    0 => format!("{major}.{minor}.{patch}"),
    _ => format!("{major}.{minor}.{patch}-pre{pre}"),
  }
}

/* ========================================================================== */

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
  let libpq_version = cx.string(libpq_version());
  let openssl_version = cx.string(openssl_version());

  cx.export_value("libpq_version", libpq_version)?;
  cx.export_value("openssl_version", openssl_version)?;

  // ===== CONNECTION ==========================================================

  cx.export_function("pq_connectdb_params", crate::conn::pq_connectdb_params)?;
  cx.export_function("pq_conninfo", crate::conn::pq_conninfo)?;
  cx.export_function("pq_set_notice_processor", crate::conn::pq_set_notice_processor)?;

  // ===== STATUS ==============================================================

  cx.export_function("pq_status", crate::conn::pq_status)?;
  cx.export_function("pq_transaction_status", crate::conn::pq_transaction_status)?;
  cx.export_function("pq_server_version", crate::conn::pq_server_version)?;
  cx.export_function("pq_error_message", crate::conn::pq_error_message)?;
  cx.export_function("pq_socket", crate::conn::pq_socket)?;
  cx.export_function("pq_backend_pid", crate::conn::pq_backend_pid)?;
  cx.export_function("pq_ssl_in_use", crate::conn::pq_ssl_in_use)?;
  cx.export_function("pq_ssl_attributes", crate::conn::pq_ssl_attributes)?;

  // ===== ASYNC ===============================================================

  cx.export_function("pq_consume_input", crate::conn::pq_consume_input)?;
  cx.export_function("pq_is_busy", crate::conn::pq_is_busy)?;
  cx.export_function("pq_setnonblocking", crate::conn::pq_setnonblocking)?;
  cx.export_function("pq_isnonblocking", crate::conn::pq_isnonblocking)?;
  cx.export_function("pq_flush", crate::conn::pq_flush)?;

// ===== ASYNCHRONOUS OPERATIONS ===============================================

  cx.export_function("pq_send_query", crate::conn::pq_send_query)?;
  cx.export_function("pq_send_query_params", crate::conn::pq_send_query_params)?;
  cx.export_function("pq_get_result", crate::conn::pq_get_result)?;

  // ===== SINGLE ROW MODE =======================================================

  cx.export_function("pq_set_single_row_mode", crate::conn::pq_set_single_row_mode)?;

  // ===== POLLING =============================================================

  cx.export_function("poll_can_write", crate::conn::poll_can_write)?;
  cx.export_function("poll_can_read", crate::conn::poll_can_read)?;

  Ok(())
}
