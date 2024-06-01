//! Main entry point.

pub mod connection;
pub mod conninfo;
pub mod debug;
pub mod errors;
pub mod ffi;
pub mod notices;
pub mod notifications;
pub mod response;

/* ========================================================================== */

/// Check is LibPQ is _thread safe_
///
/// See [PQisthreadsafe](https://www.postgresql.org/docs/current/libpq-threading.html#LIBPQ-PQISTHREADSAFE)
///
fn libpq_threadsafe() -> bool {
  unsafe { pq_sys::PQisthreadsafe() == 1 }
}


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
