use fs_err;
use std::io::Write;
use std::path::Path;

use anyhow::Context;

use super::shell;

fn append_file(dest: &Path, line: &str) -> anyhow::Result<()> {
    let mut dest_file = fs_err::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(dest)?;

    writeln!(dest_file, "{}", line)?;

    dest_file.sync_data()?;

    Ok(())
}

pub fn add(path: &std::path::Path) -> anyhow::Result<bool> {
    let aftman_dir = path.parent().unwrap();
    let env_path = aftman_dir.join("env");
    if !env_path.exists() {
        fs_err::write(
            env_path,
            include_str!("env.sh").replace("{aftman_bin}", path.to_str().unwrap()),
        )?;
    }

    for sh in shell::get_available_shells() {
        let source_cmd = sh.source_string(path.parent().unwrap())?;
        let source_cmd_with_newline = format!("\n{}", &source_cmd);

        for rc in sh.update_rcs() {
            let cmd_to_write = match fs_err::read_to_string(rc.join(&rc)) {
                Ok(contents) if contents.contains(&source_cmd) => continue,
                Ok(contents) if !contents.ends_with('\n') => &source_cmd_with_newline,
                _ => &source_cmd,
            };

            append_file(&rc, cmd_to_write)
                .with_context(|| format!("could not amend shell profile: '{}'", rc.display()))?;
        }
    }
    Ok(true)
}
