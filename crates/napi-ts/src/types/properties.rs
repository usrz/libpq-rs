use crate::napi;
use crate::types::*;

#[allow(private_bounds)]
pub trait NapiProperties<'a>: NapiTypeInternal<'a> {

  fn set_property<T: NapiType<'a>>(&self, key: &str, value: &T) -> &Self {
    let env = self.get_napi_handle().env;
    let this = self.get_napi_handle().handle;
    let key = napi::create_string_utf8(env, key);
    let value = value.get_napi_handle().handle;
    napi::set_property(env, this, key, value);
    self
  }

  fn set_property_bigint(&self, key: &str, value: impl NapiInto<NapiBigint<'a>>) -> &Self {
    let value: NapiBigint = value.napi_into(self.get_napi_handle().env);
    self.set_property(key, &value)
  }

  fn set_property_boolean(&self, key: &str, value: impl NapiInto<NapiBoolean<'a>>) -> &Self {
    let value: NapiBoolean = value.napi_into(self.get_napi_handle().env);
    self.set_property(key, &value)
  }

  fn set_property_null(&self, key: &str) -> &Self {
    let value: NapiNull = ().napi_into(self.get_napi_handle().env);
    self.set_property(key, &value)
  }

  fn set_property_number(&self, key: &str, value: impl NapiInto<NapiNumber<'a>>) -> &Self {
    let value: NapiNumber = value.napi_into(self.get_napi_handle().env);
    self.set_property(key, &value)
  }

  fn set_property_object(&self, key: &str) -> &Self {
    let value: NapiObject = ().napi_into(self.get_napi_handle().env);
    self.set_property(key, &value)
  }

  fn set_property_string<S: AsRef<str>>(&self, key: &str, value: S) -> &Self {
    let value: NapiString = value.as_ref().napi_into(self.get_napi_handle().env);
    self.set_property(key, &value)
  }

  fn set_property_symbol<S: AsRef<str>>(&self, key: &str, value: Option<S>) -> &Self {
    let value: NapiSymbol = match value {
      Some(desc) => Some(desc.as_ref()).napi_into(self.get_napi_handle().env),
      None => None.napi_into(self.get_napi_handle().env),
    };
    self.set_property(key, &value)
  }

  fn set_property_undefined(&self, key: &str) -> &Self {
    let value: NapiUndefined = ().napi_into(self.get_napi_handle().env);
    self.set_property(key, &value)
  }
}
