#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::*;

#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use unix::*;

// We should always compile this module to ensure it still builds, since we
// don't test builds on unsupported platforms.
#[allow(unused)]
mod unsupported;

#[cfg(not(any(unix, windows)))]
pub use unsupported::*;
