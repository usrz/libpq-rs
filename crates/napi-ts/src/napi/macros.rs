/// Call a NodeJS API returning a status and check it's OK or panic.
macro_rules! env_check {
  ($syscall:ident, $self:ident, $($args:expr), +) => {
    match { $syscall($self.0, $($args),+) } {
      napi_status::napi_ok => (),
      status => panic!("Error calling \"{}\": {:?}", stringify!($syscall), status),
    }
  };
}

pub (super) use env_check;
