use crate::types::*;

#[allow(private_bounds)]
pub trait NapiProperties<'a>: NapiInternal {
  fn get_property<K: AsRef<str>>(&self, key: K) -> NapiRef<'a, NapiValue> {
    let value = self.napi_handle().get_named_property(key.as_ref());
    NapiValue::from_handle(value).into()
  }

  fn set_property<K: AsRef<str>, T: NapiType + 'a>(
    &self, key: K, value: &NapiRef<'a, T>
  ) -> &Self {
    self.napi_handle().set_named_property(key.as_ref(), &value.napi_handle());
    self
  }

  fn set_property_bigint<K: AsRef<str>, V: NapiInto<'a, NapiRef<'a, NapiBigint>>>(
    &self, key: K, value: V
  ) -> &Self {
    self.set_property(&key, &value.napi_into(self.napi_env()))
  }

  fn set_property_boolean<K: AsRef<str>, V: NapiInto<'a, NapiRef<'a, NapiBoolean>>>(
    &self, key: K, value: V
  ) -> &Self {
    self.set_property(&key, &value.napi_into(self.napi_env()))
  }

  // fn set_property_external<K: AsRef<str>, V: 'static>(
  // fn set_property_function<K: AsRef<str>, V: 'static>(

  fn set_property_null<K: AsRef<str>>(&self, key: K) -> &Self {
    let value: NapiRef<NapiNull> = ().napi_into(self.napi_env());
    self.set_property(&key, &value)
  }

  fn set_property_number<K: AsRef<str>, V: NapiInto<'a, NapiRef<'a, NapiNumber>>>(
    &self, key: K, value: V
  ) -> &Self {
    self.set_property(&key, &value.napi_into(self.napi_env()))
  }

  fn set_property_string<K: AsRef<str>, V: AsRef<str>>(
    &self, key: K, value: V
  ) -> &Self {
    let value: NapiRef<NapiString> = value.as_ref().napi_into(self.napi_handle().env());
    self.set_property(key, &value)
  }

  fn set_property_symbol<K: AsRef<str>, V: AsRef<str>>(
    &self, key: K, value: Option<V>
  ) -> &Self {
    let symbol = NapiSymbolInternal::Symbol(match value {
      Some(str) => Some(str.as_ref().to_string()),
      None => None,
    });

    let value: NapiRef<NapiSymbol> = symbol.napi_into(self.napi_env());
    self.set_property(&key, &value)
  }

  fn set_property_symbol_for<K: AsRef<str>, V: AsRef<str>>(
    &self, key: K, value: V
  ) -> &Self {
    let symbol = NapiSymbolInternal::SymbolFor(value.as_ref().to_string());

    let value: NapiRef<NapiSymbol> = symbol.napi_into(self.napi_env());
    self.set_property(&key, &value)
  }

  fn set_property_undefined<K: AsRef<str>>(&self, key: K) -> &Self {
    let value: NapiRef<NapiUndefined> = ().napi_into(self.napi_env());
    self.set_property(&key, &value)
  }
}
