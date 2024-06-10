use crate::napi;
use crate::types::*;

use std::any::type_name;
use std::ops::Deref;
use std::any::TypeId;
use crate::napi::Finalizable;

struct NapiExtrnalData<T: 'static> {
  type_id: TypeId,
  pointer: *mut T,
}

impl <T: 'static> Finalizable for NapiExtrnalData<T> {
  fn finalize(self) {
    drop(unsafe { Box::from_raw(self.pointer) });
  }
}

pub struct NapiExternal<'a, T: 'static> {
  handle: NapiHandle<'a>,

  // type_id: TypeId, // this _is_ NapiExternal<T>
  // type_name: String, // the full name of NapiExternal<T>
  // reference: NapiReference, // this is potentially null for proto-objects
  pointer: *mut T, // this is the pointer to the data we boxed
}

impl <T: 'static> Debug for NapiExternal<'_, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let name = format!("NapiExternal<{}>", type_name::<T>());
    f.debug_struct(&name)
      .field("@", &self.pointer)
      .finish()
  }
}

// ===== NAPI::HANDLE CONVERSION ===============================================

impl <'a, T: 'static> NapiType<'a> for NapiExternal<'a, T> {}

impl <'a, T: 'static> NapiTypeInternal<'a> for NapiExternal<'a, T> {
  fn from_napi_handle(handle: NapiHandle<'a>) -> Result<Self, NapiErr> {
    napi::expect_type_of(handle.env, handle.handle, napi::TypeOf::napi_external)?;

    let pointer = napi::get_value_external(handle.env, handle.handle);
    let data = unsafe { &*(pointer as *mut NapiExtrnalData<T>) };

    if TypeId::of::<T>() == data.type_id {
      Ok(Self { handle, pointer: data.pointer })
    } else {
      Err(format!("Unable to downcast external value to \"{}\"", type_name::<T>()).into())
    }
  }

  fn get_napi_handle(&self) -> &NapiHandle<'a> {
    &self.handle
  }
}

// ===== EXTERNAL ==============================================================

impl <T: 'static> NapiFrom<T> for NapiExternal<'_, T> {
  fn napi_from(value: T, env: napi::Env) -> Self {
    // Create the boxed data and leak it immediately
    let boxed = Box::new(value);
    let pointer = Box::into_raw(boxed);

    let data = NapiExtrnalData {
      type_id: TypeId::of::<T>(),
      pointer,
    };

    let handle = napi::create_value_external(env, data);
    Self { handle: NapiHandle::from_napi(env, handle), pointer }
  }
}

// ===== DEREF =================================================================

impl <'a, T: 'static> Deref for NapiExternal<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.pointer }
  }
}
