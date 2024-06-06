use crate::napi;
use crate::types::*;

use std::any::type_name;
use std::any::TypeId;
use std::ops::Deref;
use std::marker::PhantomData;

type Marker = [u8; 50];
static MARKER: &Marker = b"NAPI-EXTERNAL-3F6CC3B3-AEB3-403D-AEEF-74DCBDAE7054";

struct NapiExternalLayout {
  marker: Marker, // just to make sure we get the right stuff...
  type_id: TypeId, // the type id of the payload, to check for downcasting
  type_name: String, // the type name of the payload, for debugging
  payload: *mut (), // the payload itself, as an *opaque* pointer
}

#[derive(Clone)]
pub struct NapiExternalRef {
  reference: NapiReference,
  pointer: *mut NapiExternalLayout,
}

impl Debug for NapiExternalRef {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let type_name = &unsafe { &*{self.pointer} }.type_name;
    let name = format!("NapiExternalRef<{}>", type_name);
    f.debug_struct(&name)
      .field("@", &self.reference.value())
      .finish()
  }
}

impl NapiShapeInternal for NapiExternalRef {
  fn into_napi_value(self) -> napi::Value {
    self.reference.value()
  }

  fn from_napi_value(value: napi::Value) -> Self {
    let pointer = napi::get_value_external(value) as *mut NapiExternalLayout;
    let boxed = unsafe { Box::from_raw(pointer) };

    if boxed.marker == *MARKER {
      Self { reference: value.into(), pointer: Box::into_raw(boxed) }
    } else {
      panic!("Invalid NapiExternal marker at {:?}", pointer)
    }
  }
}

impl NapiExternalRef {
  pub fn refref(&self) -> &NapiReference {
    &self.reference
  }

  pub fn downcast<T: 'static>(self) -> Option<NapiExternal<T>> {
    let type_id = unsafe { & (*self.pointer).type_id };

    if TypeId::of::<T>() == *type_id {
      Some(NapiExternal::<T> { reference: self.reference, pointer: self.pointer, phantom: PhantomData })
    } else {
      None
    }
  }
}

// ========================================================================== //

pub struct NapiExternal<T> {
  reference: NapiReference,
  pointer: *mut NapiExternalLayout,
  phantom: PhantomData<T>,
}

impl <T> Debug for NapiExternal<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let name = format!("NapiExternal<{}>", type_name::<T>());
    f.debug_struct(&name)
      .field("@", &self.reference.value())
      .finish()
  }
}

impl <T> Clone for NapiExternal<T> {
  fn clone(&self) -> Self {
    Self { reference: self.reference.clone(), pointer: self.pointer, phantom: PhantomData }
  }
}

impl <T: 'static> NapiShape for NapiExternal<T> {}

impl <T: 'static> NapiShapeInternal for NapiExternal<T> {
  fn into_napi_value(self) -> napi::Value {
    self.reference.value()
  }

  fn from_napi_value(value: napi::Value) -> Self {
    NapiExternalRef::from_napi_value(value).downcast::<T>().unwrap()
  }
}

impl <T> Into<NapiExternalRef> for NapiExternal<T> {
  fn into(self) -> NapiExternalRef {
    NapiExternalRef { reference: self.reference, pointer: self.pointer }
  }
}

impl <T> Deref for NapiExternal<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe {
      let layout = &*(self.pointer);
      &*(layout.payload as *mut T)
    }
  }
}

impl <T: 'static> NapiExternal<T> {
  pub fn new(data: T) -> Self {
    // First of all box the data payload and leak it to be handled by Node...
    // The in-memory data will contain type information for T...
    let data_boxed = Box::new(data);
    let data_pointer = Box::into_raw(data_boxed);

    // Then we prepare our layout, which we'll use to wrap our data pointer
    // (boxed, with type information) into an _untyped_ struct
    let layout: Box<NapiExternalLayout> = Box::new(NapiExternalLayout {
      marker: MARKER.to_owned(),
      type_id: TypeId::of::<T>(),
      type_name: type_name::<T>().to_owned(),
      payload: data_pointer as *mut (),
    });

    // Then we leak the whole box (again) to feed it to NodeJS
    let pointer = Box::into_raw(layout);

    // Create the external value, set up reference counting (finalization is
    // included with "create_value_external" - we just need to count clones)
    let external = napi::create_value_external(pointer);
    Self { reference: external.into(), pointer, phantom: PhantomData }
  }
}
