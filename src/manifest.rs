use std::collections::BTreeMap;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{bail, format_err, Context};
use serde::{Deserialize, Serialize};
use toml_edit::Document;

use crate::config::{config_dir, write_if_not_exists};
use crate::tool_alias::ToolAlias;
use crate::tool_id::ToolId;

pub static MANIFEST_FILE_NAME: &str = "aftman.toml";

static DEFAULT_GLOBAL_MANIFEST: &str = r#"
# This file lists tools managed by Aftman, a cross-platform toolchain manager.
# For more information, see https://github.com/LPGhatguy/aftman

[tools]
# To add a new tool, add an entry to this table.
# rojo = "rojo-rbx/rojo@6.2.0"
"#;

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub tools: BTreeMap<ToolAlias, ToolId>,

    /// The path that this manifest was loaded from if it was loaded from a file.
    #[serde(skip)]
    pub path: Option<PathBuf>,
}

impl Manifest {
    /// Create an empty global Aftman manifest if there isn't one already.
    pub fn init_global() -> anyhow::Result<()> {
        let base_dir = config_dir()?;
        fs_err::create_dir_all(&base_dir)?;

        let manifest_path = base_dir.join(MANIFEST_FILE_NAME);
        write_if_not_exists(&manifest_path, DEFAULT_GLOBAL_MANIFEST.trim())?;

        Ok(())
    }

    /// Find and load all manifests from the current directory, sorted in
    /// priority order.
    pub fn discover(mut current_dir: &Path) -> anyhow::Result<Vec<Manifest>> {
        let mut manifests = Vec::new();

        // Starting in the current directory, find every manifest file,
        // prioritizing manifests that are closer to our current dir.
        loop {
            if let Some(manifest) = Self::load_from_dir(current_dir)? {
                manifests.push(manifest);
            }

            match current_dir.parent() {
                Some(parent) => current_dir = parent,
                None => break,
            }
        }

        // We'll also load the user's global config, usually from
        // ~/.aftman/aftman.toml.
        let global_dir = config_dir()?;
        if let Some(manifest) = Self::load_from_dir(&global_dir)? {
            manifests.push(manifest);
        }

        Ok(manifests)
    }

    /// Try to load an Aftman manifest from a directory containing an
    /// aftman.toml file.
    pub fn load_from_dir(path: &Path) -> anyhow::Result<Option<Manifest>> {
        let file_path = path.join(MANIFEST_FILE_NAME);

        let contents = match fs_err::read(&file_path) {
            Ok(contents) => contents,
            Err(err) => {
                if err.kind() == io::ErrorKind::NotFound {
                    return Ok(None);
                }

                bail!(err);
            }
        };

        let mut manifest: Manifest = toml::from_slice(&contents)
            .with_context(|| format_err!("Invalid manifest at {}", file_path.display()))?;

        manifest.path = Some(file_path);

        Ok(Some(manifest))
    }

    /// Add the given alias and tool ID to the nearest manifest file.
    pub fn add_local_tool(
        mut current_dir: &Path,
        alias: &ToolAlias,
        id: &ToolId,
    ) -> anyhow::Result<()> {
        // Starting in the current directory, find every manifest file,
        // prioritizing manifests that are closer to our current dir.
        let mut manifest_path = None;
        loop {
            let file_path = current_dir.join(MANIFEST_FILE_NAME);
            if file_path.is_file() {
                manifest_path = Some(file_path);
                break;
            }

            match current_dir.parent() {
                Some(parent) => current_dir = parent,
                None => break,
            }
        }

        let manifest_path = match manifest_path {
            Some(v) => v,
            None => {
                let mut path = config_dir()?;
                path.push(MANIFEST_FILE_NAME);
                path
            }
        };

        Self::add_tool(&manifest_path, alias, id)?;

        Ok(())
    }

    pub fn add_global_tool(alias: &ToolAlias, id: &ToolId) -> anyhow::Result<()> {
        let mut manifest_path = config_dir()?;
        manifest_path.push(MANIFEST_FILE_NAME);

        Self::add_tool(&manifest_path, alias, id)?;

        Ok(())
    }

    fn add_tool(manifest_path: &Path, alias: &ToolAlias, id: &ToolId) -> anyhow::Result<()> {
        let content = fs_err::read_to_string(manifest_path)?;
        let mut document: Document = content.parse()?;
        document["tools"][alias.as_ref()] = toml_edit::value(id.to_string());

        fs_err::write(manifest_path, document.to_string())?;

        log::info!(
            "Tool {alias} = {id} has been added to {}",
            manifest_path.display()
        );

        Ok(())
    }
}
