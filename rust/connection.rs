use crate::conninfo::ConnInfo;
use crate::sys::*;
use neon::prelude::*;
use pq_sys::ConnStatusType::*;

static ENCODING_KEY: &str = "client_encoding";
static ENCODING_VAL: &str = "UTF8";

macro_rules! connection_arg_0 {
  ( $x:expr ) => {
    $x.argument::<JsBox<crate::connection::Connection>>(0)?
  };
}

pub(crate) use connection_arg_0;

#[derive(Debug)]
pub struct Connection {
  connection: *mut pq_sys::pg_conn
}

impl Drop for Connection {
  fn drop(&mut self) {
    unsafe {
      if ! self.connection.is_null() {
        println!("Finishing connection");
        pq_sys::PQfinish(self.connection);
        self.connection = std::ptr::null_mut();
      }
    }
  }
}

impl Finalize for Connection {
  fn finalize<'a, C: Context<'a>>(self, _: &mut C) {
    println!("Finalizing connection");
    drop(self);
  }
}

unsafe impl Send for Connection {}

impl Connection {
  pub fn pq_connectdb_params(info: ConnInfo) -> Result<Self, String> {
    let mut keys = Vec::<&str>::from([ ENCODING_KEY ]);
    let mut values = Vec::<&str>::from([ ENCODING_VAL ]);

    for (key, value) in info.iter() {
      // strip client encoding, we only use UTF8
      if *key == ENCODING_KEY { continue }

      // push anything else
      keys.push(key.as_str());
      values.push(value.as_str());
    }

    let k = utils::NullTerminatedArray::new(&keys);
    let v = utils::NullTerminatedArray::new(&values);

    let connection = unsafe {
      let conn = pq_sys::PQconnectdbParams(k.as_vec().as_ptr(), v.as_vec().as_ptr(), 0);
      match conn.is_null() {
        true => Err("Unable to create connection"),
        _ => Ok(Connection { connection: conn }),
      }
    }?;

    match connection.pq_status() {
      CONNECTION_OK => Ok(connection),
      _ => {
        let message = connection.pq_error_message()
          .unwrap_or("Unknown error".to_string());
        Err(message)
      }
    }
  }

  // CONNECTION: https://www.postgresql.org/docs/current/libpq-connect.html ====

  pub fn pq_conninfo(&self) -> Result<ConnInfo, String> {
    unsafe {
      ConnInfo::from_raw(pq_sys::PQconninfo(self.connection))
    }
  }

  // STATUS: https://www.postgresql.org/docs/current/libpq-status.html =========

  pub fn pq_status(&self) -> pq_sys::ConnStatusType {
    unsafe { pq_sys::PQstatus(self.connection) }
  }

  pub fn pq_transaction_status(&self) -> pq_sys::PGTransactionStatusType {
    unsafe { pq_sys::PQtransactionStatus(self.connection) }
  }

  pub fn pq_server_version(&self) -> Option<String> {
    let version = unsafe { pq_sys::PQserverVersion(self.connection) };
    match version {
      0 => None,
      _ => {
        let major = version / 10000;
        let minor = version % 10000;
        Some(format!("{major}.{minor}"))
      },
    }
  }

  pub fn pq_error_message(&self) -> Option<String> {
    let message = unsafe { pq_sys::PQerrorMessage(self.connection) };
    match message.is_null() {
      true => None,
      false => {
        let msg = utils::to_string(message)
          .unwrap_or_else(|msg| format!("Unknown error: {}", msg));
        match msg.len() {
          0 => None,
          _ => Some(msg.trim().to_string()),
        }
      },
    }
  }

  pub fn pq_socket(&self) -> i32 {
    unsafe { pq_sys::PQsocket(self.connection) }
  }

  pub fn pq_backend_pid(&self) -> i32 {
    unsafe { pq_sys::PQbackendPID(self.connection) }
  }

  pub fn pq_ssl_in_use(&self) -> bool {
    unsafe { pq_sys::PQsslInUse(self.connection) != 0 }
  }

  pub fn pq_ssl_attributes(&self) -> Result<Vec<(String, String)>, String> {
    let mut strings = Vec::<(String, String)>::new();

    unsafe {
      let raw = pq_sys::PQsslAttributeNames(self.connection);

      for x in 0.. {
        if (*raw.offset(x)).is_null() {
          break;
        } else {
          let key_ptr = *raw.offset(x);
          let val_ptr = pq_sys::PQsslAttribute(self.connection, key_ptr);
          if val_ptr.is_null() {
            continue;
          }

          let key = utils::to_string(key_ptr)?;
          let val = utils::to_string(val_ptr)?;
          strings.push((key, val));
        }
      }
    };

    Ok(strings)
  }

  // ASYNC https://www.postgresql.org/docs/current/libpq-async.html ============

  pub fn pq_consume_input(&self) -> Result<(), String> {
    let result = unsafe { pq_sys::PQconsumeInput(self.connection) };
    match result {
      1 => Ok(()),
      _ => Err(self.pq_error_message().unwrap_or("Unknown error".to_string())),
    }
  }

  // returns true when busy (1) or false wen not (0)
  pub fn pq_is_busy(&self) -> bool {
    unsafe { pq_sys::PQisBusy(self.connection) == 1 }
  }

  pub fn pq_setnonblocking(&self, nonblocking: bool) -> Result<(), String> {
    let arg = match nonblocking {
      false => 0,
      true => 1,
    };

    let result = unsafe { pq_sys::PQsetnonblocking(self.connection, arg) };

    match result {
      0 => Ok(()),
      _ => Err(self.pq_error_message().unwrap_or("Unknown error".to_string())),
    }
  }

  pub fn pq_isnonblocking(&self) -> bool {
    unsafe { pq_sys::PQisnonblocking(self.connection) == 1 }
  }

  /// returns true (0) if flushed, false (1) if data is pending, or error (-1)
  pub fn pq_flush(&self) -> Result<bool, String> {
    let result = unsafe { pq_sys::PQflush(self.connection) };
    match result {
      0 => Ok(true), // data is all flushed
      1 => Ok(false), // still some data to flush
      _ => Err(self.pq_error_message().unwrap_or("Unknown error".to_string())),
    }
  }
}
