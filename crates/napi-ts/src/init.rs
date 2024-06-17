use crate::contexts::*;
use crate::errors::*;
use crate::napi;
use crate::NapiType;

pub fn register_module<T: NapiType>(
  env: crate::napi::nodejs_sys::napi_env,
  exports: crate::napi::nodejs_sys::napi_value,
  init: fn(InitContext) -> NapiResult<T>
) -> crate::napi::nodejs_sys::napi_value {
  napi::Env::exec(env, |env| {
    let handle = napi::Handle(exports);
    let ctx = InitContext::new(env, handle);
    init(ctx).map(|exports| exports.napi_handle())
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
