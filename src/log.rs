use std::sync::atomic::{AtomicBool, Ordering};

static ENABLED: AtomicBool = AtomicBool::new(false);

pub fn enable(enabled: bool) {
    ENABLED.store(enabled, Ordering::SeqCst);
}

pub fn enabled() -> bool {
    ENABLED.load(Ordering::SeqCst)
}

#[macro_export]
macro_rules! debug {
    ($($args:tt)*) => {
        if $crate::log::enabled() {
            eprint!($($args)*)
        }
    }
}

#[macro_export]
macro_rules! debugln {
    () => ($crate::debug!("\n"));
    ($fmt:expr) => ($crate::debug!(concat!($fmt, "\n")));
    ($fmt:expr, $($args:tt)*) => ($crate::debug!(concat!($fmt, "\n"), $($args)*));
}
