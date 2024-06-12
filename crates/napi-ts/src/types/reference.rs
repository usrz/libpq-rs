// use crate::napi;
// use std::fmt::Debug;
// use crate::NapiType;
// use std::cell::Cell;
// use std::ptr;

// thread_local! {
// }

// pub trait NapiReferenceable<'a>: NapiType<'a> {
//   fn reference(&self) -> NapiReference {
//     let handle = self.napi_handle();
//     let env = handle.env();

//     let quick = FOO.get();

//     let value = env.create_reference(&handle, 1);
//     NapiReference { value }
//   }
// }

// pub struct NapiReference {
//   value: nodejs_sys::napi_ref
// }

// impl Debug for NapiReference {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     f
//       .debug_struct("NapiReference")
//       .field("@", &self.value)
//       .finish()
//   }
// }

// impl Clone for NapiReference {
//   fn clone(&self) -> Self {
//     // TODO: where is the "env" here???
//     todo!()
//   }
// }

// impl Drop for NapiReference {
//   fn drop(&mut self) {
//     // TODO: where is the "env" here???
//   }
// }

// impl NapiReference {
//   pub (super) fn handle(&self) -> napi::Handle {
//     match self.value {
//       None => panic!("Attempting to get handle from (pseudo) NapiReference"),
//       Some((handle, _)) => handle,
//     }
//   }

//   pub (super) fn expect_uninit(&self) {
//     if self.value.is_some() {
//       panic!("NapiReference already initialized")
//     }
//   }

//   // pub fn value(&self) -> NapiValue {
//   //   match self.value {
//   //     None => panic!("NapiReference not initialized"),
//   //     Some((handle, _)) =>
//   //   }

//   // }
// }
