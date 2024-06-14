use crate::types::*;
use std::any::TypeId;
use std::any::type_name;

struct NapiExtrnalData<T: 'static> {
  type_id: TypeId,
  pointer: *mut T,
}

impl <T: 'static> napi::Finalizable for NapiExtrnalData<T> {
  fn finalize(self) {
    drop(unsafe { Box::from_raw(self.pointer) });
  }
}

// ===== NAPI TYPE BASICS ======================================================

napi_type!(NapiExternal<T>, External, {
  handle: napi::Handle,
  pointer: *mut T,
});

impl <T: 'static> NapiTypeInternal for NapiExternal<T> {
  fn from_handle(handle: napi::Handle) -> Self {
    let pointer = handle.get_value_external();
    let data = unsafe { &*(pointer as *mut NapiExtrnalData<T>) };

    println!("EXTERNAL RESTORED FROM {:?}", pointer);

    if TypeId::of::<T>() == data.type_id {
      Self { handle, pointer: data.pointer }
    } else {
      panic!("Unable to downcast external value to \"{}\"", type_name::<T>())
    }
  }

  #[inline]
  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}

// ===== EXTERNAL ==============================================================

impl <T: 'static> NapiExternal<T> {
  pub fn new(env: napi::Env, data: T) -> Self {
    // Create the boxed data and leak it immediately
    let boxed = Box::new(data);
    let pointer = Box::into_raw(boxed);

    println!("EXTERNAL CREATED FROM {:?}", pointer);

    let data = NapiExtrnalData {
      type_id: TypeId::of::<T>(),
      pointer,
    };

    Self { handle: env.create_value_external(data), pointer }
  }
}

impl <T: 'static> Deref for NapiExternal<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.pointer }
  }
}
