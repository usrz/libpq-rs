use crate::napi;

use std::cell::Cell;
use std::ptr::null_mut;
use std::thread;
use crate::types::NapiBoolean;

thread_local! {
  static NAPI_ENV: Cell<napi::Env> = Cell::new(null_mut());
}

#[derive(Debug)]
pub struct Napi {}

impl Drop for Napi {
  fn drop(&mut self) {
    let env = NAPI_ENV.get();

    match env.is_null() {
      false => NAPI_ENV.set(null_mut()),
      true => panic!("NAPI environment already dropped for thread {:?}", thread::current().id())
    }
  }
}

impl Napi {
  pub(crate) unsafe fn new(env: napi::Env) -> Self {
    let old = NAPI_ENV.get();

    match old.is_null() {
      true => NAPI_ENV.set(env),
      false => panic!("NAPI environment already initialized for thread {:?}", thread::current().id())
    };

    Self {}
  }

  pub(crate) fn env() -> napi::Env {
    let env = NAPI_ENV.get();

    match env.is_null() {
      false => env,
      true => panic!("NAPI environment unavailable for thread {:?}", thread::current().id())
    }
  }

  pub fn boolean(&self, value: bool) -> NapiBoolean {
    NapiBoolean::from(value)
  }
}
