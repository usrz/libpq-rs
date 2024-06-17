mod context;
mod errors;
mod types;
mod test;


pub mod contexts;
pub mod init;
pub mod napi;

pub use context::*;
pub use errors::*;
pub use types::*;

/// Wrap the concept of a _JavaScript Type_ as given to us by NodeJS.
///
/// See [`napi_valuetype`](https://nodejs.org/api/n-api.html#napi_valuetype)
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NapiTypeOf {
  /// The JavaScript constant `undefined`.
  Undefined,
  /// The JavaScript constant `null`.
  Null,
  /// The JavaScript type `boolean`.
  Boolean,
  /// The JavaScript type `number`.
  Number,
  /// The JavaScript type `string`.
  String,
  /// The JavaScript type `symbol`.
  Symbol,
  /// The JavaScript type `object`.
  Object,
  /// The JavaScript type `function`.
  Function,
  /// Indicates a native object provided to NodeJS.
  External,
  /// The JavaScript type `bigint`.
  Bigint,
}

impl std::fmt::Display for NapiTypeOf {
  fn fmt(
    &self, fm:
    &mut std::fmt::Formatter<'_>
  ) -> std::fmt::Result {
    fm.write_str(match self {
      Self::Bigint => "Bigint",
      Self::Boolean => "Boolean",
      Self::External => "External",
      Self::Function => "Function",
      Self::Null => "Null",
      Self::Number => "Number",
      Self::Object => "Object",
      Self::String => "String",
      Self::Symbol => "Symbol",
      Self::Undefined => "Undefined",
    })
  }
}

/// A trait defining a callback from NodeJS indicating that the value
/// associated with this was garbage collected.
///
/// See [`napi_finalize`](https://nodejs.org/api/n-api.html#napi_finalize)
///
pub trait NapiFinalizable {
  fn finalize(self);
}
