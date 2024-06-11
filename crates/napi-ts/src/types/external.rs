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
  handle: napi::Handle<'a>,

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

napi_type!(NapiExternal<T>, External);

impl <'a, T: 'static> TryFrom<NapiValue<'a>> for NapiExternal<'a, T> {
  type Error = NapiErr;

  fn try_from(value: NapiValue<'a>) -> Result<Self, Self::Error> {
    let handle = match value {
      NapiValue::External(handle) => Ok::<napi::Handle, NapiErr>(handle),
      _ => Err(format!("Can't downcast {} into NapiExternal", value).into()),
    }?;

    let pointer = handle.get_value_external();
    let data = unsafe { &*(pointer as *mut NapiExtrnalData<T>) };

    println!("EXTERNAL RESTORED FROM {:?}", pointer);

    if TypeId::of::<T>() == data.type_id {
      Ok(Self { handle, pointer: data.pointer })
    } else {
      Err(format!("Unable to downcast external value to \"{}\"", type_name::<T>()).into())
    }
  }
}

// ===== EXTERNAL ==============================================================

impl <'a, T: 'static> NapiFrom<'a, T> for NapiExternal<'a, T> {
  fn napi_from(value: T, env: napi::Env<'a>) -> Self {
    // Create the boxed data and leak it immediately
    let boxed = Box::new(value);
    let pointer = Box::into_raw(boxed);

    println!("EXTERNAL CREATED FROM {:?}", pointer);

    let data = NapiExtrnalData {
      type_id: TypeId::of::<T>(),
      pointer,
    };

    Self { handle: env.create_value_external(data), pointer }
  }
}

// ===== DEREF =================================================================

impl <'a, T: 'static> Deref for NapiExternal<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.pointer }
  }
}
