use crate::contexts::*;
use crate::types::*;

#[allow(private_bounds)]
pub trait NapiProperties<'a>: NapiRefInternal {
  fn get_property<K: AsRef<str>>(&self, key: K) -> NapiRef<'a, NapiValue> {
    let value = self.napi_handle().get_named_property(key.as_ref());
    NapiValue::from_handle(value).as_napi_ref()
  }

  fn set_property<K: AsRef<str>, T: NapiType + 'a>(
    &self, key: K, value: &NapiRef<'a, T>
  ) -> &Self {
    self.napi_handle().set_named_property(key.as_ref(), &value.napi_handle());
    self
  }

  fn set_property_bigint<K: AsRef<str>, V: Into<i128>>(&self, key: K, value: V) -> &Self {
    let value = NapiBigint::new(self.napi_env(), value.into());
    self.set_property(key, &value.as_napi_ref())
  }

  fn set_property_boolean<K: AsRef<str>, V: Into<bool>>(&self, key: K, value: V) -> &Self {
    let value = NapiBoolean::new(self.napi_env(), value.into());
    self.set_property(key, &value.as_napi_ref())
  }

  fn set_property_external<K: AsRef<str>, T: 'static>(&self, key: K, data: T) -> &Self {
    let external = NapiExternal::new(self.napi_env(), data);
    self.set_property(key, &external.as_napi_ref())
  }

  fn set_property_function<K: AsRef<str>, F, T>(&self, key: K, function: F) -> &Self
  where
    F: Fn(FunctionContext) -> NapiResult<T> + 'static,
    T: NapiType,
  {
    let function = NapiFunction::new(self.napi_env(), Some(key.as_ref()), function);
    self.set_property(key, &function.as_napi_ref())
  }

  fn set_property_null<K: AsRef<str>>(&self, key: K) -> &Self {
    let value = NapiNull::new(self.napi_env());
    self.set_property(key, &value.as_napi_ref())
  }

  fn set_property_number<K: AsRef<str>, V: Into<f64>>(&self, key: K, value: V) -> &Self {
    let value = NapiNumber::new(self.napi_env(), value.into());
    self.set_property(key, &value.as_napi_ref())
  }

  fn set_property_string<K: AsRef<str>, V: AsRef<str>>(&self, key: K, value: V) -> &Self {
    let value = NapiString::new(self.napi_env(), value.as_ref());
    self.set_property(key, &value.as_napi_ref())
  }

  fn set_property_symbol<K: AsRef<str>, V: AsRef<str>>(&self, key: K, value: Option<V>) -> &Self {
    let symbol = match value {
      Some(str) => NapiSymbol::new(self.napi_env(), Some(str.as_ref())),
      None => NapiSymbol::new(self.napi_env(), None),
    };
    self.set_property(key, &symbol.as_napi_ref())
  }

  fn set_property_symbol_for<K: AsRef<str>, V: AsRef<str>>(&self, key: K, description: V) -> &Self {
    let symbol = NapiSymbol::new_for(self.napi_env(), description.as_ref());
    self.set_property(key, &symbol.as_napi_ref())
  }

  fn set_property_undefined<K: AsRef<str>>(&self, key: K) -> &Self {
    let value = NapiUndefined::new(self.napi_env());
    self.set_property(key, &value.as_napi_ref())
  }
}
