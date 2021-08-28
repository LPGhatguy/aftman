use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::{bail, format_err};
use fs_err::OpenOptions;

pub fn config_dir() -> anyhow::Result<PathBuf> {
    let mut path =
        dirs::home_dir().ok_or_else(|| format_err!("Home directory could not be found."))?;

    path.push(".aftman");
    Ok(path)
}

pub fn write_if_not_exists(path: &Path, contents: &str) -> anyhow::Result<()> {
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
