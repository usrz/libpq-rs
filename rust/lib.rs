use neon::prelude::*;

pub mod conn;
pub mod connect;
pub mod connection;
pub mod conninfo;
pub mod poll;
pub mod sys;

fn libpq_version() -> String {
  let version = unsafe { pq_sys::PQlibVersion() };
  let major = version / 10000;
  let minor = version % 10000;
  format!("{major}.{minor}")
}

fn openssl_version() -> String {
  // see https://github.com/openssl/openssl/blob/master/include/openssl/opensslv.h.in#L92
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

  // ===== CONNECT =============================================================

  // connect
  cx.export_function("pq_connectdb_params", crate::connect::pq_connectdb_params)?;

  // ===== CONN ================================================================

  // connection
  cx.export_function("pq_conninfo", crate::conn::pq_conninfo)?;
  // status
  cx.export_function("pq_status", crate::conn::pq_status)?;
  cx.export_function("pq_transaction_status", crate::conn::pq_transaction_status)?;
  cx.export_function("pq_server_version", crate::conn::pq_server_version)?;
  cx.export_function("pq_error_message", crate::conn::pq_error_message)?;
  cx.export_function("pq_socket", crate::conn::pq_socket)?;
  cx.export_function("pq_backend_pid", crate::conn::pq_backend_pid)?;
  cx.export_function("pq_ssl_in_use", crate::conn::pq_ssl_in_use)?;
  cx.export_function("pq_ssl_attributes", crate::conn::pq_ssl_attributes)?;
  // async
  cx.export_function("pq_consume_input", crate::conn::pq_consume_input)?;
  cx.export_function("pq_is_busy", crate::conn::pq_is_busy)?;
  cx.export_function("pq_setnonblocking", crate::conn::pq_setnonblocking)?;
  cx.export_function("pq_isnonblocking", crate::conn::pq_isnonblocking)?;
  cx.export_function("pq_flush", crate::conn::pq_flush)?;

  // polling
  cx.export_function("poll_can_write", crate::poll::poll_can_write)?;
  cx.export_function("poll_can_read", crate::poll::poll_can_read)?;

  Ok(())
}
