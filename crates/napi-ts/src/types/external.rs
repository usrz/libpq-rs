use crate::napi::Value;
use crate::napi;
use crate::types::*;

use std::any::Any;
use std::any::type_name;
use std::ops::Deref;
use std::ptr::null_mut;
use std::mem::transmute;
use std::any::TypeId;

#[derive(Clone)]
pub struct NapiExternalRef {
  pointer: *mut dyn Any,
  value: Value,
}

impl Debug for NapiExternalRef {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let name = format!("NapiExternalRef");
    f.debug_struct(&name)
      .field("@", &self.value)
      .finish()
  }
}

impl NapiShapeInternal for NapiExternalRef {
  fn into_napi_value(self) -> napi::Value {
    // Recreate the box... It'll drop the reference
    unsafe { Box::from_raw(self.pointer) };
    self.value
  }

  fn from_napi_value(value: napi::Value) -> Self {
    let pointer = napi::get_value_external(value);

    println!("*** FROM NAPI VALUE {:?} for {:?}", pointer, value);


    Self { pointer, value }
  }
}


// impl <T: 'static> From<NapiExternal<T>> for NapiExternalRef {
//   fn from(external: NapiExternal<T>) -> Self {
//     let value = external.reference.value();

//     // Box up our NapiExternal, with reference and whatnot!
//     let boxed = Box::new(external);
//     let pointer = Box::into_raw(boxed); // as *mut dyn Any;

//     let qqq = unsafe { &* {pointer} };

//     println!("*** REFFING IS {:?} @ {:?}", qqq, pointer);


//     // Done
//     Self { pointer, value }
//   }
// }

impl NapiExternalRef {
  pub fn downcast<T: Clone + Debug + 'static>(&self) -> Option<&T> {
    let ptr = self.pointer as *mut T;
    let value = unsafe { &* {ptr} };

    // WE HAVE TO CHECK TYPE IDs!!!!!!!

    println!("TYPE ID ON DOWNCAST {:?}", TypeId::of::<T>());

    println!("*** DOWNCASTING BOXED IS {:?} / {:?}", self.pointer, value);
    // let qqq = value as &T;
    // let www = unsafe { transmute::<T>(value) }
    Some(value)

    // match value.downcast_ref::<T>() {
    //   Some(boxed) => Some(boxed.clone()),
    //   None => None,
    // }
  }
}


// ========================================================================== //

pub struct NapiExternal<T> {
  // reference: NapiReference,
  value: napi::Value,
  pointer: *mut T,
}

impl <T> Debug for NapiExternal<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let name = format!("NapiExternal<{}>", type_name::<T>());
    f.debug_struct(&name)
      // .field("@", &self.reference.value())
      .finish()
  }
}

impl <T> Clone for NapiExternal<T> {
  fn clone(&self) -> Self {
    Self { value: self.value, pointer: self.pointer }
  }
}

impl <T: 'static> NapiShape for NapiExternal<T> {}

impl <T: 'static> NapiShapeInternal for NapiExternal<T> {
  fn into_napi_value(self) -> napi::Value {
    self.value
  }

  fn from_napi_value(value: napi::Value) -> Self {
    let pointer = napi::get_value_external(value);

    let any = unsafe { &* {pointer} };
    match any.downcast_ref::<Box<T>>() {
      Some(_) => Self { value, pointer: pointer as *mut T },
      None => panic!("Constructing NapiValue<{}> from invalid data", type_name::<T>()),
    }
  }
}

impl <T> Deref for NapiExternal<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &* {self.pointer} }
  }
}

impl <T: Debug + 'static> NapiExternal<T> {
  pub fn new(data: T) -> NapiExternal<T> {
    // Create the boxed data and leak it immediately
    let boxed = Box::new(data);
    let pointer = Box::into_raw(boxed);

    // TODO: WE NEED TO EMBED TYPE ID IN THE NAPIEXTERNAL (SLEDGEHAMMER)
    println!("TYPE ID ON CONSTRUCTION {:?}", TypeId::of::<NapiExternal<T>>());

    // TODO: WE NEED TO CREATE THIS WITH A "FAKE" REF... WE'LL PUT THE
    // *REAL* ONE IN WHEN WE COME BACK FROM MEMORY... (get_external_data)
    let sss = Self { value: null_mut(), pointer };
    let boxed_self = Box::new(sss);
    let pointer_self = Box::into_raw(boxed_self);


    // Now create Node's "external" object
    let value = napi::create_value_external(pointer_self);
    let xxx = unsafe { &* {pointer_self} };

    println!("*** CONSTRUCTING BOXED IS {:?} @ {:?} for {:?}", xxx, pointer_self, value);

    Self { value, pointer }

    // let www = xxx.clone();
    // www.pointer = value;

    // Set ourselves up..
  }
}
