use crate::napi;
use crate::types::*;

pub struct NapiNull {
  handle: napi::Handle,
}

// ===== NAPI TYPE BASICS ======================================================

// napi_type!(NapiNull, Null);
napi_value!(NapiNull, Null);

impl NapiTypeInternal for NapiNull {
  fn from_handle(handle: napi::Handle) -> Self {
    Self { handle }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
}


// impl TryFrom<&NapiValue> for NapiNull {
//   type Error = NapiErr;

//   fn try_from(value: &NapiValue) -> Result<Self, Self::Error> {
//     match value {
//       NapiValue::Null(handle) => Ok(NapiNull::from_handle(*handle)),
//       _ => Err(format!("Unable to downcast {} into {}", value, stringify!(NapiNull)).into())
//     }
//   }
// }


// ===== CONVERSION IN =========================================================

impl <'a> NapiFrom<'a, ()> for NapiRef<'a, NapiNull> {
  fn napi_from(_: (), env: napi::Env) -> Self {
    NapiNull { handle: env.get_null() }.into()
  }
}
