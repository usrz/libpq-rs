use crate::types::*;

#[allow(private_bounds)]
pub trait NapiProperties<'a>: NapiType<'a> {
  fn get_property<K: AsRef<str>>(&self, key: K) -> NapiRef<NapiValue> {
    let value = self.napi_handle().get_named_property(key.as_ref());
    NapiValue::from_handle(value).as_napi_ref()
  }

  fn set_property<K: AsRef<str>, T: NapiType<'a>>(
    &self, key: K, value: &NapiRef<'a, T>
  ) -> &Self {
    self.napi_handle().set_named_property(key.as_ref(), &value.napi_handle());
    self
  }

  fn set_property_bigint<K: AsRef<str>, V: Into<i128>>(&self, key: K, value: V) -> &Self {
    let value = NapiBigint::new(value.into());
    self.set_property(key, &value.as_napi_ref())
  }

  fn set_property_boolean<K: AsRef<str>, V: Into<bool>>(&self, key: K, value: V) -> &Self {
    let value = NapiBoolean::new(value.into());
    self.set_property(key, &value.as_napi_ref())
  }

  fn set_property_external<K: AsRef<str>, T: 'static>(&self, key: K, data: T) -> &Self {
    let external = NapiExternal::new(data);
    self.set_property(key, &external.as_napi_ref())
  }

  fn set_property_null<K: AsRef<str>>(&self, key: K) -> &Self {
    let value = NapiNull::new();
    self.set_property(key, &value.as_napi_ref())
  }

  fn set_property_number<K: AsRef<str>, V: Into<f64>>(&self, key: K, value: V) -> &Self {
    let value = NapiNumber::new(value.into());
    self.set_property(key, &value.as_napi_ref())
  }

  fn set_property_string<K: AsRef<str>, V: AsRef<str>>(&self, key: K, value: V) -> &Self {
    let value = NapiString::new(value.as_ref());
    self.set_property(key, &value.as_napi_ref())
  }

  fn set_property_symbol<K: AsRef<str>, V: AsRef<str>>(&self, key: K, value: Option<V>) -> &Self {
    let symbol = match value {
      Some(str) => NapiSymbol::new(Some(str.as_ref())),
      None => NapiSymbol::new(None),
    };
    self.set_property(key, &symbol.as_napi_ref())
  }

  fn set_property_symbol_for<K: AsRef<str>, V: AsRef<str>>(&self, key: K, description: V) -> &Self {
    let symbol = NapiSymbol::new_for(description.as_ref());
    self.set_property(key, &symbol.as_napi_ref())
  }

  fn set_property_undefined<K: AsRef<str>>(&self, key: K) -> &Self {
    let value = NapiUndefined::new();
    self.set_property(key, &value.as_napi_ref())
  }
}
