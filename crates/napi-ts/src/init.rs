pub unsafe fn napi_init(
  env: napi_env,
  exports: napi_value,
  init: impl FnOnce() -> () // impl FnOnce(&Napi, NapiObject) -> NapiResult<NapiObject> + 'static,
) -> napi_value {
  println("NAPI INITIALIZING!");

  // let napi = Napi::from_env(env);

  // let result: Result<Result<NapiObject, NapiError>, NapiError> = {
  //   let exports = NapiObject::from_napi_value(&napi, exports);
  //   match exports {
  //     Ok(result) => Ok(init(&napi, result)),
  //     Err(error) => Err(error),
  //   }
  //   // let exports2 = exports?;
  //   // init(&napi, exports)
  // };

  // // match result {
  // //   Ok(result) => result.as_napi_value(),
  // //   Err(error) => error.throw(&napi)
  // // }

  // // result.or_else(|error| error.throw(&napi))
  // // .or_else(|error| panic!("Unable to throw error: {}", error.message))
  // // .unwrap();

  exports
}
