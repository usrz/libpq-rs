mod errors;
mod functions;
mod objects;
mod primitives;
mod references;

pub use errors::*;
pub use functions::*;
pub use objects::*;
pub use primitives::*;
pub use references::*;

pub type CallbackInfo = nodejs_sys::napi_callback_info;
pub type Env = nodejs_sys::napi_env;
pub type Reference = nodejs_sys::napi_ref;
pub type Status = nodejs_sys::napi_status;
pub type Value = nodejs_sys::napi_value;
pub type ValueType = nodejs_sys::napi_valuetype;

/// Call a NodeJS API returning a status and check it's OK or panic.
macro_rules! napi_check {
  ($syscall:ident, $($args:expr), +) => {
    match { $syscall(crate::env::Napi::env(), $($args),+) } {
      Status::napi_ok => (),
      status => panic!("Error calling \"{}\": {:?}", stringify!($syscall), status),
    }
  };
}

// Publish "napi_check" to our modules
pub(self) use napi_check;