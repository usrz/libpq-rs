use crate::contexts::*;
use crate::errors::*;
use crate::napi;

pub fn register_module(
  env: crate::napi::nodejs_sys::napi_env,
  exports: crate::napi::nodejs_sys::napi_value,
  init: fn(InitContext) -> Result<(), NapiErr>
) -> crate::napi::nodejs_sys::napi_value {
  napi::Env::exec(env, |env| {
    let handle = napi::Handle(exports);
    init(InitContext::new(env, handle))
      .map_err(|err| err.into_handle())
      .map(|_| handle)
  })
}

#[macro_export]
macro_rules! napi_init {
  ($initializer:expr) => {
    #[no_mangle]
    extern "C" fn napi_register_module_v1(
      env: ::napi_ts::napi::nodejs_sys::napi_env,
      exports: ::napi_ts::napi::nodejs_sys::napi_value,
    ) -> ::napi_ts::napi::nodejs_sys::napi_value {
      ::napi_ts::init::register_module(env, exports, $initializer)
    }
  };
}
