//! Main entry point.

use napi_ts::*;

use ffi::to_string_lossy;
use context::Context;
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
/// See [OpenSSL_version](https://github.com/openssl/openssl/blob/master/include/openssl/opensslv.h.in#L92)
/// See [OPENSSL_FULL_VERSION_STRING](https://github.com/openssl/openssl/blob/master/include/openssl/crypto.h.in#L166)
fn openssl_version() -> String {
  // We lifted "OPENSSL_FULL_VERSION_STRING" (7) directly from OpenSSL
  let version = unsafe { openssl_sys::OpenSSL_version(7) };
  to_string_lossy(version).unwrap()
}

/* ========================================================================== */

#[derive(Debug)]
struct Foobar {
  s: String
}

unsafe impl Send for Foobar { }

napi_ts::napi_init!(|ctx, exports| {
  println!("Initializing...");
  println!("  openssl version: {}", openssl_version());
  println!("    libpq version: {} (threadsafe={})", libpq_version(), libpq_threadsafe());

  let str = ctx.string("foobar");

  exports
    .set_property_string("openssl_version", openssl_version())
    .set_property_string("libpq_version", libpq_version())
    .set_property_boolean("libpq_threadsafe", libpq_threadsafe())
  ;

  ctx.function(move |cx, _, _| {
    // println!("{:?}", str);
    let str1 = cx.string("foo");

    cx.function(move |cx2, _, _| {
      // println!("{:?}", str);
      // println!("{:?}", str1);

      Ok(cx2.string("foo"))
    });

    Ok(cx.string("foo"))
  });

  Ok(())
});
