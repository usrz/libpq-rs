pub mod errors;
pub mod init;
pub mod env;
pub mod napi;
pub mod types;

pub use errors::*;
pub use types::*;

#[macro_export]
macro_rules! napi_init {
  ($initializer:expr) => {
    #[no_mangle]
    extern "C" fn napi_register_module_v1(
      env: ::napi_ts::napi::Env,
      exports: ::napi_ts::napi::Handle,
    ) -> ::napi_ts::napi::Handle {
      ::napi_ts::init::register_module(env, exports, $initializer)
    }
  };
}
