use std::env::{consts::EXE_SUFFIX, current_exe};
use std::path::PathBuf;
use std::process::Command;

use anyhow::Context;

use crate::config::config_dir;
use crate::tool_alias::ToolAlias;
use crate::tool_id::ToolId;

pub struct ToolStorage {
    storage_dir: PathBuf,
    bin_dir: PathBuf,
}

impl ToolStorage {
    pub fn init() -> anyhow::Result<Self> {
        let base_dir = config_dir()?;

        let storage_dir = base_dir.join("tool-storage");
        fs_err::create_dir_all(&storage_dir)?;

        let bin_dir = base_dir.join("bin");
        fs_err::create_dir_all(&bin_dir)?;

        Ok(Self {
            storage_dir,
            bin_dir,
        })
    }

    pub fn add(&self, alias: &ToolAlias, id: &ToolId) -> anyhow::Result<()> {
        self.install(id)?;
        self.link(alias)?;
        Ok(())
    }

    pub fn run(&self, id: &ToolId, args: Vec<String>) -> anyhow::Result<i32> {
        eprintln!("Run {} with args {:?}", id, args);

        self.install(id)?;

        let mut exe_path = self.exe_dir(id);
        exe_path.push(exe_name(id));

        let status = Command::new(exe_path).args(args).status().unwrap();

        Ok(status.code().unwrap_or(1))
    }

    fn install(&self, id: &ToolId) -> anyhow::Result<()> {
        let dir = self.exe_dir(id);
        let exe_path = dir.join(exe_name(id));

        if exe_path.exists() {
            return Ok(());
        }

        todo!("actually install this tool: {}", id);
    }

    fn link(&self, alias: &ToolAlias) -> anyhow::Result<()> {
        let self_path =
            current_exe().context("Failed to discover the name of the Aftman executable")?;

        let link_name = format!("{}{}", alias.as_ref(), EXE_SUFFIX);
        let link_path = self.bin_dir.join(link_name);

        fs_err::copy(self_path, link_path).context("Failed to create Aftman alias")?;
        Ok(())
    }

    fn exe_dir(&self, id: &ToolId) -> PathBuf {
        let mut dir = self.storage_dir.clone();
        dir.push(id.name().scope());
        dir.push(id.name().name());
        dir
    }
}

fn exe_name(id: &ToolId) -> String {
    format!("{}{}", id.version(), EXE_SUFFIX)
}
