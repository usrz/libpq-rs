#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
  ($($arg:tt)*) => {{ println!(">>> LIBPQ DEBUG >>> {}", format!($($arg)*)) }}
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
  ($($arg:tt)*) => {{}}
}
