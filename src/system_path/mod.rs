#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::add;

#[cfg(not(target_os = "windows"))]
pub fn add(path: &std::path::Path) -> anyhow::Result<bool> {
    log::debug!("Not adding value to path because this platform is not supported.");
    Ok(false)
}
