use std::path::PathBuf;

use anyhow::format_err;

pub fn config_dir() -> anyhow::Result<PathBuf> {
    let mut path =
        dirs::home_dir().ok_or_else(|| format_err!("Home directory could not be found."))?;

    path.push(".aftman");
    Ok(path)
}
