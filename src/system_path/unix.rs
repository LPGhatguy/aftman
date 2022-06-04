use std::io::{self, Write};
use std::path::Path;

use anyhow::Context;
use fs_err::OpenOptions;

use crate::config::write_if_not_exists;
use crate::home::Home;

const SHELL_TEMPLATE: &str = include_str!("./env.sh");

pub fn init(home: &Home) -> anyhow::Result<()> {
    let env_path = home.path().join("env");
    let bin_dir = home.bin_dir();
    let bin_dir = bin_dir
        .to_str()
        .context("bin directory has invalid Unicode")?;

    let body = SHELL_TEMPLATE.replace("{our_bin_dir}", bin_dir);

    write_if_not_exists(&env_path, &body)?;

    Ok(())
}

pub fn add(home: &Home) -> anyhow::Result<bool> {
    let mut env_path = home.path_str();
    env_path.push_str("/env");

    let source_str = format!(r#". "{env_path}""#);

    let mut added_any = false;
    if let Some(home) = dirs::home_dir() {
        for filename in [
            ".profile",
            ".bash_profile",
            ".bashrc",
            ".bash_login",
            ".zshenv",
        ] {
            let path = home.join(filename);
            let added = append_line_if_not_present(&path, &source_str)?;
            added_any |= added;
        }
    }

    Ok(added_any)
}

fn append_line_if_not_present(path: &Path, line: &str) -> anyhow::Result<bool> {
    let ends_with_newline = match fs_err::read_to_string(path) {
        // This file already has this line, skip it.
        Ok(contents) if contents.contains(line) => return Ok(false),
        Ok(contents) if contents.ends_with('\n') => true,
        _ => false,
    };

    let mut file = match OpenOptions::new().create_new(false).append(true).open(path) {
        Ok(file) => file,
        Err(err) => {
            if err.kind() != io::ErrorKind::NotFound {
                log::error!("Error trying to write {}: {err}", path.display());
            }

            return Ok(false);
        }
    };

    if !ends_with_newline {
        write!(file, "\n")?;
    }

    writeln!(file, "{}", line)?;
    file.sync_data()?;

    Ok(true)
}
