use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::tool_name::ToolName;
use crate::tool_spec::ToolSpec;

pub static MANIFEST_FILE_NAME: &str = "aftman.toml";
pub static DEFAULT_GLOBAL_MANIFEST: &str = r#"
# This file lists tools managed by Aftman, a cross-platform toolchain manager.
# For more information, see https://github.com/LPGhatguy/aftman

[tools]
# To add a new tool, add an entry to this table.
# rojo = "rojo-rbx/rojo@6.2.0"
"#
.trim();

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub tools: BTreeMap<ToolName, ToolSpec>,
}

impl Manifest {
    /// Find and load all manifests from the current directory, sorted in
    /// priority order.
    pub fn discover(current_dir: &Path) -> anyhow::Result<Vec<Manifest>> {
        todo!()
    }
}
