use crate::napi;

use std::cell::Cell;
use std::ptr::null_mut;
use std::thread;

thread_local! {
  static NAPI_ENV: Cell<napi::Env> = Cell::new(null_mut());
}

#[derive(Debug)]
pub struct Napi {
  env: napi::Env,
  prev: napi::Env,
}

impl Napi {
  pub(crate) fn new(env: napi::Env) -> Self {
    let prev = NAPI_ENV.get();
    NAPI_ENV.set(env);

    Self { env, prev }
  }

  pub(crate) fn env() -> napi::Env {
    let env = NAPI_ENV.get();

    match env.is_null() {
      true => panic!("NAPI environment unavailable for thread {:?}", thread::current().id()),
      false => env
    }
  }
}

impl Drop for Napi {
  fn drop(&mut self) {
    let env = NAPI_ENV.get();

    match self.env == env {
      false => panic!("NAPI environment misalignment in thread {:?}", thread::current().id()),
      true => NAPI_ENV.set(self.prev)
    }
  }
}
