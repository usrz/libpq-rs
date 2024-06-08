use crate::napi;
use crate::types::*;

use nodejs_sys::napi_value;
use std::any::TypeId;
use std::any::type_name;
use std::ops::Deref;
use std::ptr;

#[derive(Clone)]
pub struct NapiExternalRef {
  handle: napi::Handle,
}

impl Debug for NapiExternalRef {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let name = format!("NapiExternalRef");
    f.debug_struct(&name)
      .field("@", &self.handle)
      .finish()
  }
}

impl NapiShapeInternal for NapiExternalRef {
  fn into_napi_value(self) -> napi::Handle {
    self.handle
  }

  fn from_napi_value(handle: napi::Handle) -> Self {
    napi::expect_type_of(handle, napi::Type::napi_external);
    Self { handle }
  }
}

impl NapiExternalRef {
  pub(super) unsafe fn downcast<T: NapiShape + 'static>(&self) -> NapiResult<T> {
    // Get the data from NodeJS and refrence it immediately for downcasting...
    let pointer = napi::get_value_external(self.handle) as *mut T;
    let referenced = unsafe { &* {pointer} };

    // Call the "downcast_external" on the (hopefully) NapiExternal<_>
    referenced.downcast_external::<T>(self.handle)
  }
}

// ========================================================================== //

pub struct NapiExternal<T: 'static> {
  type_id: TypeId, // this _is_ NapiExternal<T>
  type_name: String, // the full name of NapiExternal<T>
  reference: NapiReference, // this is potentially null for proto-objects
  pointer: *mut T, // this is the pointer to the data we boxed
}

impl <T: 'static> Debug for NapiExternal<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let name = format!("NapiExternal<{}>", type_name::<T>());
    f.debug_struct(&name)
      .field("@", &self.pointer)
      .finish()
  }
}

impl <T: 'static> Clone for NapiExternal<T> {
  fn clone(&self) -> Self {
    Self {
      type_id: self.type_id.clone(),
      type_name: self.type_name.clone(),
      reference: self.reference.clone(),
      pointer: self.pointer.clone(),
    }
  }
}

impl <T: 'static> napi::Finalizable for NapiExternal<T> {
  fn finalize(self) {
    // NOTE: we can not get rid of this, and just rely on when our "napi_value"
    // inside of the reference is null... We clone the "prototype" NapiExternal
    // several times, and all those clones will be dropped individually!
    drop(unsafe { Box::from_raw(self.pointer) });
  }
}

impl <T: 'static> NapiShape for NapiExternal<T> {}

impl <T: 'static> NapiShapeInternal for NapiExternal<T> {
  unsafe fn downcast_external<T2: NapiShape + 'static>(&self, value: napi::Handle) -> NapiResult<Self> {
    // When the type ID of what we want is _NOT_ the type ID of what we store,
    // decline conversion... Someone asked for a downcasting to NapiShape<X>,
    // while here we're holding data for NapiShape<Y>...
    if TypeId::of::<T2>() != self.type_id {
      return Err(format!("Unable to downcast {:?} into {:?}", self.type_name, type_name::<T2>()).into())
    }

    // If we're holding a napi_value and napi_reference, then we're doing
    // something *incredibly* wrong... This should only be called when we're
    // being created from data held by NodeJS, and that never ever has them!
    if ! self.reference.handle().is_null() { panic!("NapiExternal already initialized") }

    // Great, we're pretty sure we can create ourselves... This is basically a
    // clone operation, injecting a brand new NapiReference in the instance.
    Ok(Self {
      type_id: self.type_id.clone(),
      type_name: self.type_name.clone(),
      reference: value.into(), // Look, ma! New reference!
      pointer: self.pointer.clone(),
    })
  }

  fn into_napi_value(self) -> napi::Handle {
    self.reference.handle()
  }

  fn from_napi_value(value: napi::Handle) -> Self {
    unsafe { NapiExternalRef::from_napi_value(value).downcast::<Self>().unwrap() }
  }
}

impl <T: 'static> Deref for NapiExternal<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &* {self.pointer} }
  }
}

impl <T: 'static> NapiExternal<T> {
  pub fn new(data: T) -> NapiExternal<T> {
    // Create the boxed data and leak it immediately
    let boxed = Box::new(data);
    let pointer = Box::into_raw(boxed);

    // Here "external" is the data held by NodeJS itself. It has no "reference"
    // itself (null pointer as NodeJS keeps its own internal reference counts)
    // but it holds on to the pointer derived from the data we're externalizing.
    let external = Self {
      type_id: TypeId::of::<NapiExternal<T>>(),
      type_name: type_name::<NapiExternal<T>>().to_string(),
      reference: (ptr::null_mut() as napi_value).into(),
      pointer,
    };

    // Now create Node's "external", which is a full "NapiExternal<T>" object.
    // We need this to properly address downcasting from "NapiValue"...
    let value = napi::create_value_external(external);

    // Here comes the fun part: we have given to Node ownership of the first
    // "NapiExternal<T>" (and related data pointer), we now do a _clone_ of
    // that (remember, null napi_value in there) and inject a new NapiReference
    // object in it to let Node track it via its internal reference counts.
    NapiExternal::<T>::from_napi_value(value)
  }
}
