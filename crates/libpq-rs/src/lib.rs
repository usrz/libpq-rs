//! Main entry point.

use napi_ts::*;

use ffi::to_string_lossy;
use conninfo::PQConninfo;
use context::NapiContext;
use context::MainContext;
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

napi_ts::napi_init!(|mut env, exports| {
  println!("Initializing...");
  println!("  openssl version: {}", openssl_version());
  println!("    libpq version: {} (threadsafe={})", libpq_version(), libpq_threadsafe());

  // let info = Foobar { s: "foobar BAAZ".to_string() };
  // let external = NapiExternal::new(info);
  // println!("EXTERNAL IS {:?}", external);

  // let _f = NapiFunction::named("my great function", move |_, args| {
  //   args[2].downcast::<NapiFunction>()
  //     .and_then(|value| value.call(&[ NapiNull::new() ]))?;

  //   NapiReturn::void()
  // });

  // let _f2 = NapiFunction::named("another function", |_, args| {
  //   // println!("REAL THE EXTERNAL IS {:?}", external);
  //   println!("ARG FROM EXTERNAL IS {:?}", args[0]);
  //   let qqqq = args[0].downcast::<NapiExternal<Foobar>>().unwrap();
  //   println!("FROM ARGUMENT {:?}", qqqq);
  //   println!("FROM ARGUMENT {:?}", qqqq.s);
  //   // qqqq.iter().for_each(|(key, val)| {
  //   //   println!("DEREFERENCED {} => {}", key, val);
  //   // });

  //   println!("SECOND CALLBACK!!!");
  //   NapiReturn::void()
  // });


  // let _s1 = NapiSymbol::new("foobar");
  // let _s2 = NapiSymbol::symbol_for("foobar");
  // println!("S1 {:?}", _s1.description());
  // println!("S2 {:?}", _s2.description());

  exports
  //   .set_property("foo", &_f)
  //   .set_property("bar", &_f2)
  //   .set_property("external", &external)
    .set_property_string("openssl_version", openssl_version())
    .set_property_string("libpq_version", libpq_version())
    .set_property_boolean("libpq_threadsafe", libpq_threadsafe())
    .set_property("baz",&env.object())
  ;

  Ok(exports)
});


// pub fn test(env: &mut MainContext) {
//   env.bigint(123);
//   // NapiBigint
// }
