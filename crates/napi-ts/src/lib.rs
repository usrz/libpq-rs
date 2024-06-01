#[macro_export]
macro_rules! napi_init {
  ($initializer:expr) => {
    #[no_mangle]
    unsafe extern "C" fn napi_register_module_v1(
      _env: napi_sys::napi_env,
      exports: napi_sys::napi_value,
    ) -> napi_sys::napi_value {
      unsafe { napi_ts::init::napi_init(env, exports, $initializer) }
    }
  };
}
