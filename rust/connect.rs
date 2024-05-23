use crate::connection::Connection;
use crate::conninfo::ConnInfo;
use crate::sys::*;
use neon::prelude::*;

pub fn pq_connectdb_params(mut cx: FunctionContext) -> JsResult<JsPromise> {
  let arg = cx.argument_opt(0)
    .unwrap_or(cx.undefined().as_value(&mut cx));

  let info = match types::js_type_of(arg, &mut cx) {
    // use defaults
    types::JsTypeOf::JsNull |
    types::JsTypeOf::JsUndefined => {
      ConnInfo::from_defaults().or_else(| message | cx.throw_error(message))
    },
    // use string dsn
    types::JsTypeOf::JsString => {
      let string = arg.downcast_or_throw::<JsString, _>(&mut cx)?;
      ConnInfo::from_str(&string.value(&mut cx))
      .or_else(| msg | cx.throw_error(msg))
    },
    // use any other _plain_ object
    types::JsTypeOf::JsObject => {
      let object = arg.downcast_or_throw::<JsObject, _>(&mut cx)?;
      ConnInfo::from_object(&mut cx, object)
    }
    // nothing else is valid
    _ => {
      let ptype = types::js_type_of(arg, &mut cx);
      cx.throw_error(format!("Invalid argument (0) of type \"{}\"", ptype))
    }
  }?;

  let promise = cx.task( || {
    let connection = Connection::pq_connectdb_params(info)?;

    connection.pq_setnonblocking(true)?;
    match connection.pq_isnonblocking() {
      false => Err("Unable to set non-blocking status".to_string()),
      true => Ok(connection),
    }
  }).promise(move | mut cx, result | {
    let connection = result
      .or_else(| msg | cx.throw_error(msg))?;

    let boxed = cx.boxed(connection);

    Ok(boxed)


  });

  Ok(promise)
}
