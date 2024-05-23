use neon::prelude::*;

#[derive(Debug)]
pub enum JsTypeOf {
  // primitives
  JsBigInt,
  JsBoolean,
  JsNull,
  JsNumber,
  JsString,
  JsUndefined,
  // standard objects
  JsArray,
  JsDate,
  JsError,
  JsFunction,
  JsPromise,
  // buffers, array buffers, typed arrays
  JsBuffer,
  JsArrayBuffer,
  JsUint8Array,
  JsInt8Array,
  JsUint16Array,
  JsInt16Array,
  JsUint32Array,
  JsInt32Array,
  JsBigUint64Array,
  JsBigInt64Array,
  JsFloat32Array,
  JsFloat64Array,
  // plain objects
  JsObject,
  // anything else is unknown
  Unknown,
}

impl std::fmt::Display for JsTypeOf {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // fmt::Debug::fmt(&self, f)
    match self {
      // primitives
      JsTypeOf::JsBigInt => write!(f, "bigint"),
      JsTypeOf::JsBoolean => write!(f, "boolean"),
      JsTypeOf::JsNull => write!(f, "null"),
      JsTypeOf::JsNumber => write!(f, "number"),
      JsTypeOf::JsString => write!(f, "string"),
      JsTypeOf::JsUndefined => write!(f, "undefined"),
      // standard objects
      JsTypeOf::JsArray => write!(f, "Array"),
      JsTypeOf::JsDate => write!(f, "Date"),
      JsTypeOf::JsError => write!(f, "Error"),
      JsTypeOf::JsFunction => write!(f, "Function"),
      JsTypeOf::JsPromise => write!(f, "Promise"),
      // buffers, array buffers, typed arrays
      JsTypeOf::JsBuffer => write!(f, "Buffer"),
      JsTypeOf::JsArrayBuffer => write!(f, "ArrayBuffer"),
      JsTypeOf::JsUint8Array => write!(f, "Uint8Array"),
      JsTypeOf::JsInt8Array => write!(f, "Int8Array"),
      JsTypeOf::JsUint16Array => write!(f, "Uint16Array"),
      JsTypeOf::JsInt16Array => write!(f, "Int16Array"),
      JsTypeOf::JsUint32Array => write!(f, "Uint32Array"),
      JsTypeOf::JsInt32Array => write!(f, "Int32Array"),
      JsTypeOf::JsBigUint64Array => write!(f, "BigUint64Array"),
      JsTypeOf::JsBigInt64Array => write!(f, "BigInt64Array"),
      JsTypeOf::JsFloat32Array => write!(f, "Float32Array"),
      JsTypeOf::JsFloat64Array => write!(f, "Float64Array"),
      // plain objects
      JsTypeOf::JsObject => write!(f, "object"),
      // anything else is unknown
      _ => write!(f, "[unknown]"),
    }
  }
}

pub fn js_type_of<'a, V: Value, C: Context<'a>>(value: Handle<V>, cx: &mut C) -> JsTypeOf {
  // primitives
  if value.is_a::<neon::types::JsBigInt, _>(cx) { return JsTypeOf::JsBigInt }
  if value.is_a::<neon::types::JsBoolean, _>(cx) { return JsTypeOf::JsBoolean }
  if value.is_a::<neon::types::JsNull, _>(cx) { return JsTypeOf::JsNull }
  if value.is_a::<neon::types::JsNumber, _>(cx) { return JsTypeOf::JsNumber }
  if value.is_a::<neon::types::JsString, _>(cx) { return JsTypeOf::JsString }
  if value.is_a::<neon::types::JsUndefined, _>(cx) { return JsTypeOf::JsUndefined }

  // standard objects
  if value.is_a::<neon::types::JsArray, _>(cx) { return JsTypeOf::JsArray }
  if value.is_a::<neon::types::JsDate, _>(cx) { return JsTypeOf::JsDate }
  if value.is_a::<neon::types::JsError, _>(cx) { return JsTypeOf::JsError }
  if value.is_a::<neon::types::JsFunction, _>(cx) { return JsTypeOf::JsFunction }
  if value.is_a::<neon::types::JsPromise, _>(cx) { return JsTypeOf::JsPromise }

  // buffers, array buffers, typed arrays
  if value.is_a::<neon::types::JsBuffer, _>(cx) { return JsTypeOf::JsBuffer }
  if value.is_a::<neon::types::JsArrayBuffer, _>(cx) { return JsTypeOf::JsArrayBuffer }
  if value.is_a::<neon::types::JsBigInt64Array, _>(cx) { return JsTypeOf::JsBigInt64Array }
  if value.is_a::<neon::types::JsBigUint64Array, _>(cx) { return JsTypeOf::JsBigUint64Array }
  if value.is_a::<neon::types::JsFloat32Array, _>(cx) { return JsTypeOf::JsFloat32Array }
  if value.is_a::<neon::types::JsFloat64Array, _>(cx) { return JsTypeOf::JsFloat64Array }
  if value.is_a::<neon::types::JsInt16Array, _>(cx) { return JsTypeOf::JsInt16Array }
  if value.is_a::<neon::types::JsInt32Array, _>(cx) { return JsTypeOf::JsInt32Array }
  if value.is_a::<neon::types::JsInt8Array, _>(cx) { return JsTypeOf::JsInt8Array }
  if value.is_a::<neon::types::JsUint16Array, _>(cx) { return JsTypeOf::JsUint16Array }
  if value.is_a::<neon::types::JsUint32Array, _>(cx) { return JsTypeOf::JsUint32Array }
  if value.is_a::<neon::types::JsUint8Array, _>(cx) { return JsTypeOf::JsUint8Array }

  // plain objects
  if value.is_a::<neon::types::JsObject, _>(cx) { return JsTypeOf::JsObject }

  // anything else is unknown
  JsTypeOf::Unknown
}
