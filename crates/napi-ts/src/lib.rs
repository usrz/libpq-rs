pub mod errors;
pub mod init;
pub mod env;
pub mod napi;
pub mod types;

pub use env::Napi;

#[macro_export]
macro_rules! napi_init {
  ($initializer:expr) => {
    #[no_mangle]
    extern "C" fn napi_register_module_v1(
      env: napi_ts::init::nodejs_sys::napi_env,
      exports: napi_ts::init::nodejs_sys::napi_value,
    ) -> napi_ts::init::nodejs_sys::napi_value {
      napi_ts::init::napi_init(env, exports, $initializer)
    }
  };
}
