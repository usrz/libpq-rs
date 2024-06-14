use crate::context::*;
use crate::errors::*;
use crate::napi;
use crate::types::*;

pub fn register_module(
  env: crate::napi::nodejs_sys::napi_env,
  exports: crate::napi::nodejs_sys::napi_value,
  init: fn(Context, NapiRef<NapiObject>) -> Result<(), NapiErr>
) -> crate::napi::nodejs_sys::napi_value {
  napi::Env::exec(env, |env| {
    let ctx = Context::new(env);
    let handle = env.handle(exports);
    let object = NapiRef::<NapiObject>::from_handle(handle);
    init(ctx, object).map(|_| handle)
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
