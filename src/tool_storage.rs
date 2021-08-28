use crate::config::config_dir;

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
}
