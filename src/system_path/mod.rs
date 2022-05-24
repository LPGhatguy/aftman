#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::add;

#[cfg(unix)]
mod shell;

#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use unix::add;

#[cfg(all(not(windows), not(unix)))]
pub fn add(path: &std::path::Path) -> anyhow::Result<bool> {
    log::debug!("Not adding value to path because this platform is not supported.");
    Ok(false)
}
