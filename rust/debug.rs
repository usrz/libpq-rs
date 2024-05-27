use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

/// Emit a debug message (when `debug_assertions` are enabled)
///
#[cfg(debug_assertions)]
macro_rules! debug {
  ($($arg:tt)*) => {{
    println!(">>> LIBPQ DEBUG >>> {}", format!($($arg)*))
  }}
}

/// Emit a debug message (when `debug_assertions` are enabled)
///
#[cfg(not(debug_assertions))]
macro_rules! debug {
  ($($arg:tt)*) => {{}}
}

/// Emit a debug message when creating an instance
///
macro_rules! debug_create {
  ($arg:expr) => {{
    let this = $arg;
    debug!("Created {:?}", this);
    this
  }}
}

/// Emit a debug message when dropping an instance
///
macro_rules! debug_drop {
  ($arg:expr) => {
    debug!("Dropping {:?}", $arg)
  }
}

/// Implement the [`Debug`] trait including only a single field
///
macro_rules! debug_self {
  ($t:ty, $field:ident) => {
    debug_self!($t, $field, "id");
  };

  ($t:ty, $field:ident, $name:expr) => {
    impl std::fmt::Debug for $t {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>().rsplit_once(":").unwrap().1)
          .field($name, &self.$field)
          .finish()
      }
    }
  };
}

pub(crate) use debug;
pub(crate) use debug_create;
pub(crate) use debug_drop;
pub(crate) use debug_self;

static DEBUG_ID: AtomicUsize = AtomicUsize::new(1);

/// Create a new unique debugging identifier
///
pub fn debug_id() -> usize {
  DEBUG_ID.fetch_add(1, Ordering::Relaxed)
}
