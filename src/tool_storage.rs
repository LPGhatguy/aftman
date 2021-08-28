use std::env::consts::EXE_SUFFIX;
use std::path::PathBuf;
use std::process::Command;

use crate::config::config_dir;
use crate::tool_alias::ToolAlias;
use crate::tool_id::ToolId;

pub struct ToolStorage {}

impl ToolStorage {
    pub fn init() -> anyhow::Result<()> {
        let base_dir = config_dir()?;

        let storage_dir = base_dir.join("tool-storage");
        fs_err::create_dir_all(storage_dir)?;

        let bin_dir = base_dir.join("bin");
        fs_err::create_dir_all(bin_dir)?;

        Ok(())
    }

    pub fn add(alias: &ToolAlias, id: &ToolId) -> anyhow::Result<()> {
        install(id)?;
        link(alias, id)?;
        Ok(())
    }

    pub fn run(id: &ToolId, args: Vec<String>) -> anyhow::Result<i32> {
        install(id)?;

        let mut exe_path = exe_dir(id)?;
        exe_path.push(exe_name(id));

        let status = Command::new(exe_path).args(args).status().unwrap();

        Ok(status.code().unwrap_or(1))
    }
}

fn exe_dir(id: &ToolId) -> anyhow::Result<PathBuf> {
    let mut dir = config_dir()?;
    dir.push("tool-storage");
    dir.push(id.name().scope());
    dir.push(id.name().name());
    Ok(dir)
}

fn exe_name(id: &ToolId) -> String {
    format!("{}{}", id.version(), EXE_SUFFIX)
}

fn install(id: &ToolId) -> anyhow::Result<()> {
    let dir = exe_dir(id)?;
    let exe_path = dir.join(exe_name(id));

    if exe_path.exists() {
        return Ok(());
    }

    todo!("actually install this tool: {}", id);
}

fn link(alias: &ToolAlias, id: &ToolId) -> anyhow::Result<()> {
    todo!()
}
