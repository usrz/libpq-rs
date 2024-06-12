use crate::types::*;

#[allow(private_bounds)]
pub trait NapiProperties<'a>: NapiType<'a> {

  fn get_property<K: AsRef<str>>(&self, key: K) -> NapiValue<'a> {
    let this = self.napi_handle();
    let env = this.env();

    let key = env.create_string_utf8(key.as_ref());
    let handle = env.get_property(&this, &key);

    NapiValue::from(handle)
  }

  fn set_property<K: AsRef<str>, V: NapiType<'a>>(
    &self, key: K, value: &V
  ) -> &Self {
    let this = self.napi_handle();
    let env = this.env();

    let key = env.create_string_utf8(key.as_ref());
    let value = value.napi_handle();

    this.set_property(&key, &value);
    self
  }

  fn set_property_bigint<K: AsRef<str>, V: NapiInto<'a, NapiBigint<'a>>>(
    &self, key: K, value: V
  ) -> &Self {
    let value: NapiBigint = value.napi_into(self.napi_handle().env());
    self.set_property(key, &value)
  }

  fn set_property_boolean<K: AsRef<str>, V: NapiInto<'a, NapiBoolean<'a>>>(
    &self, key: K, value: V
  ) -> &Self {
    let value: NapiBoolean = value.napi_into(self.napi_handle().env());
    self.set_property(key, &value)
  }

  fn set_property_external<K: AsRef<str>, V: 'static>(
    &self, key: K, value: V
  ) -> &Self {
    let value: NapiExternal<V> = value.napi_into(self.napi_handle().env());
    self.set_property(key, &value)
  }

  fn set_property_null<K: AsRef<str>>(&self, key: K) -> &Self {
    let value: NapiNull = ().napi_into(self.napi_handle().env());
    self.set_property(key, &value)
  }

  fn set_property_number<K: AsRef<str>, V: NapiInto<'a, NapiNumber<'a>>>(
    &self, key: K, value: V
  ) -> &Self {
    let value: NapiNumber = value.napi_into(self.napi_handle().env());
    self.set_property(key, &value)
  }

  fn set_property_object<K: AsRef<str>>(
    &self, key: K
  ) -> &Self {
    let value: NapiObject = ().napi_into(self.napi_handle().env());
    self.set_property(key, &value)
  }

  fn set_property_string<K: AsRef<str>, V: AsRef<str>>(
    &self, key: K, value: V
  ) -> &Self {
    let value: NapiString = value.as_ref().napi_into(self.napi_handle().env());
    self.set_property(key, &value)
  }

  fn set_property_symbol<K: AsRef<str>, V: AsRef<str>>(
    &self, key: K, value: Option<V>
  ) -> &Self {
    let symbol = NapiSymbolInternal::Symbol(match value {
      Some(str) => Some(str.as_ref().to_string()),
      None => None,
    });

    let value: NapiSymbol = symbol.napi_into(self.napi_handle().env());
    self.set_property(key, &value)
  }

  fn set_property_symbol_for<K: AsRef<str>, V: AsRef<str>>(
    &self, key: K, value: V
  ) -> &Self {
    let symbol = NapiSymbolInternal::SymbolFor(value.as_ref().to_string());

    let value: NapiSymbol = symbol.napi_into(self.napi_handle().env());
    self.set_property(key, &value)
  }

  fn set_property_undefined<K: AsRef<str>>(
    &self, key: K
  ) -> &Self {
    let value: NapiUndefined = ().napi_into(self.napi_handle().env());
    self.set_property(key, &value)
  }
}
