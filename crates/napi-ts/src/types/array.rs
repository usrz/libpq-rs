use crate::types::*;
use std::cell::OnceCell;

// ===== NAPI TYPE BASICS ======================================================

pub struct NapiArray<'a> {
  phantom: PhantomData<&'a ()>,
  handle: napi::Handle,
  pop: OnceCell<napi::Handle>,
  push: OnceCell<napi::Handle>,
  splice: OnceCell<napi::Handle>,
}

napi_type!(NapiArray, Object, {
  unsafe fn from_handle(handle: napi::Handle) -> Result<Self, NapiErr> {
    if ! handle.is_array() {
      return Err("Specified object is not a JavaScript Array".into())
    } else {
      Ok(Self {
        phantom: PhantomData,
        handle,
        pop: OnceCell::new(),
        push: OnceCell::new(),
        splice: OnceCell::new(),
      })
    }
  }

  fn napi_handle(&self) -> napi::Handle {
    self.handle
  }
});

// impl <'a> NapiProperties<'a> for NapiArray<'a> {}

// ===== ARRAY =================================================================

impl <'a> NapiArray<'a> {
  pub fn new() -> Self {
    unsafe { Self::from_handle(napi::env().create_array()).unwrap() }
  }

  pub fn length(&self) -> u32 {
    let value = self.handle.get_named_property("length");
    value.get_value_double() as u32
  }

  // ===== ELEMENT OPS =========================================================

  pub fn get_element(&self, index: u32) -> NapiRef<'a, NapiValue> {
    let value = self.handle.get_element(index);
    NapiValue::from_handle(value).as_napi_ref()
  }

  pub fn set_element<T: NapiType<'a>>(
    &self, index: u32, value: &NapiRef<'a, T>
  ) -> &Self {
    self.handle.set_element(index, &value.napi_handle());
    self
  }

  pub fn has_element(&self, index: u32) -> bool {
    self.handle.has_element(index)
  }

  pub fn delete_element(&self, index: u32) {
    self.handle.delete_element(index)
  }

  // ===== PUSH ================================================================

  pub fn push<T: NapiType<'a>>(&self, item: &NapiRef<'a, T>) -> u32 {
    let push = self.push.get_or_init(|| self.handle.get_named_property("push"));
    let result = push.call_function(&self.handle, &[&item.napi_handle()]).unwrap();
    napi::env().get_value_double(&result) as u32
  }

  pub fn pushn(&self, items: &[&NapiRef<'a, NapiValue<'a>>]) -> u32 {
    let push = self.push.get_or_init(|| self.handle.get_named_property("push"));

    let handles: Vec<napi::Handle> = items
      .into_iter()
      .map(|arg| arg.napi_handle())
      .collect();
    let ehandles: Vec<&napi::Handle> = handles.iter().collect();

    let result = push.call_function(&self.handle, ehandles.as_slice()).unwrap();
    napi::env().get_value_double(&result) as u32
  }

  // ===== POP =================================================================

  pub fn pop(&'a self) -> NapiRef<'a, NapiValue> {
    let push = self.pop.get_or_init(|| self.handle.get_named_property("pop"));
    let result = push.call_function(&self.handle, &[]).unwrap();
    NapiValue::from_handle(result).as_napi_ref()
  }

  // ===== SPLICE ==============================================================

  pub fn splice(
    &self,
    start: u32,
    delete_count: u32,
    items: &[&NapiRef<'a, NapiValue<'a>>],
  ) {
    let splice = self.splice.get_or_init(|| self.handle.get_named_property("splice"));
    let start = napi::env().create_double(start as f64);
    let delete_count = napi::env().create_double(delete_count as f64);

    let mut handles: Vec<napi::Handle> = items
      .into_iter()
      .map(|arg| arg.napi_handle())
      .collect();
    handles.insert(0, delete_count); // this will become args[1]
    handles.insert(0, start); // this stays as args[0]

    let ehandles: Vec<&napi::Handle> = handles.iter().collect();

    splice.call_function(&self.handle, ehandles.as_slice()).unwrap();
    // TODO: return the deleted handles!
  }
}
