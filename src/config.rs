use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::{bail, format_err};
use fs_err::OpenOptions;

use crate::manifest::{DEFAULT_GLOBAL_MANIFEST, MANIFEST_FILE_NAME};

pub fn config_dir() -> anyhow::Result<PathBuf> {
    let mut path =
        dirs::home_dir().ok_or_else(|| format_err!("Home directory could not be found."))?;

    path.push(".aftman");
    Ok(path)
}

/// Attempt to initialize Aftman's global configuration files so that users can
/// edit them by hand.
pub fn initialize_global_config() -> anyhow::Result<()> {
    let base_dir = config_dir()?;
    fs_err::create_dir_all(&base_dir)?;

    let manifest_path = base_dir.join(MANIFEST_FILE_NAME);
    write_if_not_exists(&manifest_path, DEFAULT_GLOBAL_MANIFEST.trim())?;

    Ok(())
}

fn write_if_not_exists(path: &Path, contents: &str) -> anyhow::Result<()> {
    let mut file = match OpenOptions::new().create_new(true).write(true).open(path) {
        Ok(file) => BufWriter::new(file),
        Err(err) => {
            if err.kind() == io::ErrorKind::AlreadyExists {
                return Ok(());
            }

            bail!(err);
        }
    };

    file.write_all(contents.as_bytes())?;

    Ok(())
}
