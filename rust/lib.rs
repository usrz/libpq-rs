use neon::prelude::*;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

pub mod bindings;
pub mod connection;
pub mod conninfo;
pub mod errors;
pub mod ffi;
pub mod notices;
pub mod notifications;
pub mod response;

/* ========================================================================== */

/// Emit a debug message (when `debug_assertions` are enabled)
///
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
  ($($arg:tt)*) => {{
    println!(">>> LIBPQ DEBUG >>> {}", format!($($arg)*))
  }}
}

/// Emit a debug message (when `debug_assertions` are enabled)
///
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
  ($($arg:tt)*) => {{}}
}

/// Emit a debug message when creating an instance
///
#[macro_export]
macro_rules! debug_create {
  ($arg:expr) => {{
    let this = $arg;
    debug!("Created {:?}", this);
    this
  }}
}

/// Emit a debug message when dropping an instance
///
#[macro_export]
macro_rules! debug_drop {
  ($arg:expr) => {
    debug!("Dropping {:?}", $arg)
  }
}

/// Implement the [`Debug`] trait including only a single field
///
#[macro_export]
macro_rules! debug_self {
  ($t:ty, $field:ident) => {
    debug_self!($t, $field, "id");
  };

  ($t:ty, $field:ident, $name:expr) => {
    impl Debug for $t {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(type_name::<Self>().rsplit_once(":").unwrap().1)
          .field($name, &self.$field)
          .finish()
      }
    }
  };
}

static DEBUG_ID: AtomicU64 = AtomicU64::new(1);

/// Create a new unique debugging identifier
///
pub fn debug_id() -> u64 {
  DEBUG_ID.fetch_add(1, Ordering::Relaxed)
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

  /* ======================================================================== *
   * CONNECTION                                                               *
   * ======================================================================== */

  cx.export_function("pq_connectdb_params", crate::bindings::pq_connectdb_params)?;
  cx.export_function("pq_conninfo", crate::bindings::pq_conninfo)?;
  cx.export_function("pq_set_notice_processor", crate::bindings::pq_set_notice_processor)?;

  // ===== STATUS ==============================================================

  cx.export_function("pq_status", crate::bindings::pq_status)?;
  cx.export_function("pq_transaction_status", crate::bindings::pq_transaction_status)?;
  cx.export_function("pq_server_version", crate::bindings::pq_server_version)?;
  cx.export_function("pq_error_message", crate::bindings::pq_error_message)?;
  cx.export_function("pq_socket", crate::bindings::pq_socket)?;
  cx.export_function("pq_backend_pid", crate::bindings::pq_backend_pid)?;
  cx.export_function("pq_ssl_in_use", crate::bindings::pq_ssl_in_use)?;
  cx.export_function("pq_ssl_attributes", crate::bindings::pq_ssl_attributes)?;

  // ===== ASYNC ===============================================================

  cx.export_function("pq_consume_input", crate::bindings::pq_consume_input)?;
  cx.export_function("pq_is_busy", crate::bindings::pq_is_busy)?;
  cx.export_function("pq_setnonblocking", crate::bindings::pq_setnonblocking)?;
  cx.export_function("pq_isnonblocking", crate::bindings::pq_isnonblocking)?;
  cx.export_function("pq_flush", crate::bindings::pq_flush)?;

// ===== ASYNCHRONOUS OPERATIONS ===============================================

  cx.export_function("pq_send_query", crate::bindings::pq_send_query)?;
  cx.export_function("pq_send_query_params", crate::bindings::pq_send_query_params)?;
  cx.export_function("pq_get_result", crate::bindings::pq_get_result)?;

  // ===== SINGLE ROW MODE =======================================================

  cx.export_function("pq_set_single_row_mode", crate::bindings::pq_set_single_row_mode)?;

  // ===== POLLING =============================================================

  cx.export_function("poll_can_write", crate::bindings::poll_can_write)?;
  cx.export_function("poll_can_read", crate::bindings::poll_can_read)?;

  /* ======================================================================== *
   * RESPONSE                                                                 *
   * ======================================================================== */

  cx.export_function("pq_result_status", crate::bindings::pq_result_status)?;
  cx.export_function("pq_result_error_essage", crate::bindings::pq_result_error_essage)?;
  cx.export_function("pq_cmd_status", crate::bindings::pq_cmd_status)?;
  cx.export_function("pq_cmd_tuples", crate::bindings::pq_cmd_tuples)?;
  cx.export_function("pq_ntuples", crate::bindings::pq_ntuples)?;
  cx.export_function("pq_nfields", crate::bindings::pq_nfields)?;
  cx.export_function("pq_fname", crate::bindings::pq_fname)?;
  cx.export_function("pq_ftype", crate::bindings::pq_ftype)?;
  cx.export_function("pq_getisnull", crate::bindings::pq_getisnull)?;
  cx.export_function("pq_getvalue", crate::bindings::pq_getvalue)?;

  // ===== WRAPPING ============================================================

  cx.export_function("unwrap_response", crate::bindings::unwrap_response)?;

  Ok(())
}
