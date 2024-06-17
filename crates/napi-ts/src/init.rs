use crate::*;
use crate::contexts::*;
use crate::napi;
use std::marker::PhantomData;

pub struct NapiInit<'a> {
  phantom: PhantomData<&'a ()>,
  env: crate::napi::nodejs_sys::napi_env,
  exports: crate::napi::nodejs_sys::napi_value,
}

impl <'a> NapiInit<'a> {
  pub fn new(
    env: napi::nodejs_sys::napi_env,
    exports: napi::nodejs_sys::napi_value,
  ) -> Self {
    Self { phantom: PhantomData, env, exports }
  }

  pub fn init<'b, F, R>(&self, function: F) -> napi::nodejs_sys::napi_value
  where
    'a: 'b,
    F: (Fn(InitContext<'b>) -> NapiResult2<'b, R>) + 'static,
    R: NapiType<'b> + 'b,
  {
    napi::Env::exec(self.env, move |_| {
      let handle = napi::Handle(self.exports);
      let context = InitContext::new(handle);

      function(context)
        .map_err(|err| err.into_handle())
        .map(|ok| ok.napi_handle())
    })
  }
}

#[macro_export]
macro_rules! napi_init {
  ($initializer:expr) => {
    #[no_mangle]
    extern "C" fn napi_register_module_v1<'a>(
      env: ::napi_ts::napi::nodejs_sys::napi_env,
      exports: ::napi_ts::napi::nodejs_sys::napi_value,
    ) -> ::napi_ts::napi::nodejs_sys::napi_value {
      // Establish the <'a> lifetime tying it to our "NapiInit"
      let init = ::napi_ts::init::NapiInit::<'a>::new(env, exports);
      // Run the initialized in the constrained <'a> lifetime
      init.init($initializer)
    }
  };
}
