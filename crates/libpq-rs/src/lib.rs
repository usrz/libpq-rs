//! Main entry point.

use napi_ts::*;

use ffi::to_string_lossy;
use std::error::Error;
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
  value: String
}

impl Foobar {
  fn new(value: &str) -> Self {
    Self { value: value.to_owned() }
  }
}

unsafe impl Send for Foobar { }





napi_ts::napi_init!(|cx| {
  let exports = cx.exports();

  println!("Initializing...");
  println!("  openssl version: {}", openssl_version());
  println!("    libpq version: {} (threadsafe={})", libpq_version(), libpq_threadsafe());

  let foo = cx.string("foo");
  // let foo = exports.get_property("shuster");
  // let bar = exports.get_property("f").downcast::<NapiFunction>()?.call(None, &[])?;
  // let baz = exports.get_property("a").downcast::<NapiArray>()?.pop();
  // let obj = exports.get_property("o").downcast::<NapiObject>()?;

  let foo = cx.function(move |cx| {
    // println!("{:?}", foo);
    // println!("{:?}", exports);
    // println!("{:?}", baz);
    println!("-> foo -> THIS {:?}", cx.this());
    println!("-> foo -> ARGS {:?}", cx.args());
    let function = cx.arg(2).downcast::<NapiFunction>()?;

    let rezzo = function
      .with(cx.string("shuster"))
      .with(&cx.bigint(123))
      .call();

    rezzo
  });

  let bar = cx.function(move |cx| {
    println!("-> bar -> THIS {:?}", cx.this());
    println!("-> bar -> ARGS {:?}", cx.args());
    Ok(cx.undefined())
  });

  exports
    .set_property_string("openssl_version", openssl_version())
    .set_property_string("libpq_version", libpq_version())
    .set_property_boolean("libpq_threadsafe", libpq_threadsafe())
    .set_property("foo", &foo)
    .set_property("bar", &bar)
  ;

  println!("\n\n\n------------------------------------------------------");

  let arr = cx.array();
  println!("ARRAY {:?}", arr);
  println!("{}", arr.length());
  arr.set_element(0, &cx.string("xxx"));
  arr.set_element(12, &cx.string("xxx"));
  println!("{}", arr.length());
  arr.delete_element(12);
  arr.delete_element(0);
  arr.push(&cx.string("shii"));
  arr.pushn(&[&cx.string("shoo").as_value(), &cx.string("shaa").as_value()]);
  arr.splice(0, 13, &[]);

  exports.set_property("arr", &arr);
  println!("------------------------------------------------------\n\n\n");

  Ok(cx.exports())
});
