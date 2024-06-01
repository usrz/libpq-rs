//! Main entry point.

// use neon::prelude::*;
// use ctor::ctor;
// use neon::prelude;

pub mod bindings;
pub mod connection;
pub mod conninfo;
pub mod debug;
pub mod errors;
pub mod ffi;
pub mod notices;
pub mod notifications;
pub mod response;
pub mod runner;


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

// #[neon::main]
// fn main(mut cx: ModuleContext) -> NeonResult<()> {
//   if ! libpq_threadsafe() {
//     cx.throw_error("Sorry, your LibPQ is NOT thread safe")?;
//   }

//   /* ===== VERSIONS ========================================================= */

//   let libpq_version = cx.string(libpq_version());
//   let openssl_version = cx.string(openssl_version());

//   cx.export_value("libpq_version", libpq_version)?;
//   cx.export_value("openssl_version", openssl_version)?;

//   /* ======================================================================== *
//    * CONNECTION                                                               *
//    * ======================================================================== */

//   cx.export_function("pq_connectdb_params", crate::bindings::pq_connectdb_params)?;
//   cx.export_function("pq_conninfo", crate::bindings::pq_conninfo)?;
//   cx.export_function("pq_set_notice_processor", crate::bindings::pq_set_notice_processor)?;

//   // ===== STATUS ==============================================================

//   cx.export_function("pq_status", crate::bindings::pq_status)?;
//   cx.export_function("pq_transaction_status", crate::bindings::pq_transaction_status)?;
//   cx.export_function("pq_server_version", crate::bindings::pq_server_version)?;
//   cx.export_function("pq_error_message", crate::bindings::pq_error_message)?;
//   cx.export_function("pq_socket", crate::bindings::pq_socket)?;
//   cx.export_function("pq_backend_pid", crate::bindings::pq_backend_pid)?;
//   cx.export_function("pq_ssl_in_use", crate::bindings::pq_ssl_in_use)?;
//   cx.export_function("pq_ssl_attributes", crate::bindings::pq_ssl_attributes)?;

//   // ===== ASYNC ===============================================================

//   cx.export_function("pq_consume_input", crate::bindings::pq_consume_input)?;
//   cx.export_function("pq_is_busy", crate::bindings::pq_is_busy)?;
//   cx.export_function("pq_setnonblocking", crate::bindings::pq_setnonblocking)?;
//   cx.export_function("pq_isnonblocking", crate::bindings::pq_isnonblocking)?;
//   cx.export_function("pq_flush", crate::bindings::pq_flush)?;

// // ===== ASYNCHRONOUS OPERATIONS ===============================================

//   cx.export_function("pq_send_query", crate::bindings::pq_send_query)?;
//   cx.export_function("pq_send_query_params", crate::bindings::pq_send_query_params)?;
//   cx.export_function("pq_get_result", crate::bindings::pq_get_result)?;

//   // ===== SINGLE ROW MODE =======================================================

//   cx.export_function("pq_set_single_row_mode", crate::bindings::pq_set_single_row_mode)?;

//   // ===== POLLING =============================================================

//   cx.export_function("poll_can_write", crate::bindings::poll_can_write)?;
//   cx.export_function("poll_can_read", crate::bindings::poll_can_read)?;

//   /* ======================================================================== *
//    * RESPONSE                                                                 *
//    * ======================================================================== */

//   cx.export_function("pq_result_status", crate::bindings::pq_result_status)?;
//   cx.export_function("pq_result_error_message", crate::bindings::pq_result_error_message)?;
//   cx.export_function("pq_cmd_status", crate::bindings::pq_cmd_status)?;
//   cx.export_function("pq_cmd_tuples", crate::bindings::pq_cmd_tuples)?;
//   cx.export_function("pq_ntuples", crate::bindings::pq_ntuples)?;
//   cx.export_function("pq_nfields", crate::bindings::pq_nfields)?;
//   cx.export_function("pq_fname", crate::bindings::pq_fname)?;
//   cx.export_function("pq_ftype", crate::bindings::pq_ftype)?;
//   cx.export_function("pq_getisnull", crate::bindings::pq_getisnull)?;
//   cx.export_function("pq_getvalue", crate::bindings::pq_getvalue)?;

//   // ===== WRAPPING ============================================================

//   cx.export_function("unwrap_response", crate::bindings::unwrap_response)?;

//   /* ======================================================================== *
//    * RUNNERS                                                                  *
//    * ======================================================================== */

//    cx.export_function("runner_create", crate::runner::runner_create)?;
//    cx.export_function("runner_query", crate::runner::runner_query)?;
//    cx.export_function("runner_query_params", crate::runner::runner_query_params)?;

//    Ok(())
// }

// fn napi_start(napi: &Napi, exports: NapiObject) -> NapiResult<NapiObject> {
//   let libpq_version = libpq_version();
//   let openssl_version = openssl_version();
//   let is_threadsafe = libpq_threadsafe();

//   let version = napi.string(&libpq_version)?;
//   exports.set_property(napi, "libpq_version", version)?;

//   Ok(exports)
// }

// thread_local! {
//   static NAPI_ENV: Cell<napi_sys::napi_env> = Cell::new(null_mut());
// }


// fn init2() -> () {
//   println!("WE HAVE BEEN CALLED!!! {:?}", thread::current().id());
// }

macro_rules! init_me {
  ($initializer:expr) => {
    #[no_mangle]
    unsafe extern "C" fn napi_register_module_v1(
      _env: napi_sys::napi_env,
      exports: napi_sys::napi_value,
    ) -> napi_sys::napi_value {
      $initializer();
      exports
    }
  };
}

// init_me!(init2);

// init_me!(|| {
//   println!("CLOSURE CALLED!!! {:?}", thread::current().id());
// });


// // #[no_mangle]
// unsafe extern "C" fn napi_register_module_v1(
//   env: napi_sys::napi_env,
//   exports: napi_sys::napi_value,
// ) -> napi_sys::napi_value {
//   println!("PTR IS {:?} => {:?}", env, std::thread::current());

//   NAPI_ENV.set(env);
//   let foo = NAPI_ENV.get();

//   // let code = CString::new("THECODE").unwrap();
//   // let str = CString::new("Hello, world!").unwrap();
//   // napi_throw_error(env, code.as_ptr(), str.as_ptr());

//   // let mut my_bool: bool = false;
//   // let my_bool_ptr = &mut my_bool as *mut bool;

//   // napi_is_exception_pending(env, my_bool_ptr);
//   // println!("EXCEPTION PENDING... {}", my_bool);
//   // napi_fatal_error(
//   //   "location".as_ptr() as *const std::ffi::c_char,
//   //   8,
//   //   "message".as_ptr() as *const std::ffi::c_char,
//   //   7);
//   // return napi::napi_init(env, exports, napi_start)
//   exports
// }
