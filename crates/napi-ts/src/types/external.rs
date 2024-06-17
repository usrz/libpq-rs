use crate::NapiFinalizable;
use crate::types::*;
use std::any::TypeId;
use std::any::type_name;

struct NapiExtrnalData<T: 'static> {
  type_id: TypeId,
  pointer: *mut T,
}

impl <T: 'static> NapiFinalizable for NapiExtrnalData<T> {
  fn finalize(self) {
    drop(unsafe { Box::from_raw(self.pointer) });
  }
}

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiExternal<'a, T> {
  phantom: PhantomData<&'a ()>,
  handle: napi::Handle,
  pointer: *mut T,
}

napi_type!(NapiExternal<T>, External, {
  unsafe fn from_handle(handle: napi::Handle) -> Result<Self, NapiErr> {
    let pointer = handle.get_value_external();
    let data = &*(pointer as *mut NapiExtrnalData<T>);

    if TypeId::of::<T>() == data.type_id {
      Ok(Self { phantom: PhantomData, handle, pointer: data.pointer })
    } else {
      Err(format!("Unable to downcast external value into \"{}\"", type_name::<T>()).into())
    }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});

// ===== EXTERNAL ==============================================================

impl <'a, T: 'static> NapiExternal<'a, T> {
  pub fn new(env: napi::Env, data: T) -> Self {
    // Create the boxed data and leak it immediately
    let boxed = Box::new(data);
    let pointer = Box::into_raw(boxed);

    println!("EXTERNAL CREATED FROM {:?}", pointer);

    let data = NapiExtrnalData {
      type_id: TypeId::of::<T>(),
      pointer,
    };

    Self { phantom: PhantomData, handle: env.create_value_external(data), pointer }
  }
}

impl <'a, T: 'static> Deref for NapiExternal<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.pointer }
  }
}
