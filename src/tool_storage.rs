use std::borrow::Cow;
use std::collections::BTreeSet;
use std::env::current_dir;
use std::env::{consts::EXE_SUFFIX, current_exe};
use std::fmt::Write;
use std::io::{self, BufWriter, Read};
use std::io::{Seek, Write as _};
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context};
use command_group::CommandGroup;
use fs_err::File;
use once_cell::unsync::OnceCell;

use crate::config::config_dir;
use crate::manifest::Manifest;
use crate::tool_alias::ToolAlias;
use crate::tool_id::ToolId;
use crate::tool_source::{Asset, GitHubSource, Release};
use crate::tool_spec::ToolSpec;
use crate::trust::TrustCache;

pub struct ToolStorage {
    pub storage_dir: PathBuf,
    pub bin_dir: PathBuf,
    github: OnceCell<GitHubSource>,
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
            github: OnceCell::new(),
        })
    }

    pub fn add(
        &self,
        spec: &ToolSpec,
        alias: Option<&ToolAlias>,
        global: bool,
    ) -> anyhow::Result<()> {
        let current_dir = current_dir().context("Failed to find current working directory")?;

        let alias = match alias {
            Some(alias) => Cow::Borrowed(alias),
            None => Cow::Owned(ToolAlias::new(spec.name().name())?),
        };

        let id = self.install_inexact(spec)?;
        self.link(&alias)?;

        if global {
            Manifest::add_global_tool(&alias, &id)?;
        } else {
            Manifest::add_local_tool(&current_dir, &alias, &id)?;
        }

        Ok(())
    }

    pub fn run(&self, id: &ToolId, args: Vec<String>) -> anyhow::Result<i32> {
        self.install_exact(id)?;

        let exe_path = self.exe_path(id);
        let status = Command::new(exe_path).args(args).group_status().unwrap();

        Ok(status.code().unwrap_or(1))
    }

    pub fn update_links(&self) -> anyhow::Result<()> {
        let self_path =
            current_exe().context("Failed to discover path to the Aftman executable")?;

        log::info!("Updating all Aftman binaries...");

        for entry in fs_err::read_dir(&self.bin_dir)? {
            let entry = entry?;
            let path = entry.path();

            fs_err::copy(&self_path, path)?;
        }

        let aftman_path = self.bin_dir.join(format!("aftman{}", EXE_SUFFIX));
        fs_err::copy(&self_path, aftman_path)?;

        log::info!("Updated Aftman binaries successfully!");

        Ok(())
    }

    /// Ensure a tool that matches the given spec is installed.
    fn install_inexact(&self, spec: &ToolSpec) -> anyhow::Result<ToolId> {
        let installed_path = self.storage_dir.join("installed.txt");
        let installed = InstalledToolsCache::read(&installed_path)?;

        let trusted_path = self.storage_dir.join("trusted.txt");
        let trusted = TrustCache::read(&trusted_path)?;
        let is_trusted = trusted.tools.contains(spec.name());

        if !is_trusted {
            if atty::isnt(atty::Stream::Stderr) {
                bail!(
                    "Tool {} has never been installed and is not a trusted tool. \
                     Run `aftman add {}` in your terminal to install it and trust this tool.",
                    spec.name(),
                    spec
                );
            }

            let proceed = dialoguer::Confirm::new()
                .with_prompt(format!(
                    "Tool {} has never been installed before. Trust this tool?",
                    spec.name()
                ))
                .interact_opt()?;

            if let Some(false) | None = proceed {
                eprintln!(
                    "Skipping installation of {} and exiting with an error.",
                    spec
                );
                std::process::exit(1);
            }

            TrustCache::add(&trusted_path, spec.name().clone())?;
        }

        log::info!("Installing tool: {}", spec);

        log::debug!("Fetching GitHub releases...");
        let github = self.github.get_or_init(GitHubSource::new);
        let mut releases = github.get_all_releases(spec.name())?;
        releases.sort_by(|a, b| a.version.cmp(&b.version).reverse());

        log::trace!("All releases found: {:#?}", releases);
        log::debug!("Choosing a release...");

        for release in &releases {
            // If we've requested a version, skip any releases that don't match
            // the request.
            if let Some(requested_version) = spec.version() {
                if requested_version != &release.version {
                    continue;
                }
            }

            let id = ToolId::new(spec.name().clone(), release.version.clone());

            if installed.tools.contains(&id) {
                log::debug!("Tool is already installed.");
                return Ok(id);
            }

            let compatible_assets = self.get_compatible_assets(&release);
            if compatible_assets.is_empty() {
                log::warn!(
                    "Version {} was compatible, but had no assets compatible with your platform.",
                    release.version
                );
                continue;
            }

            if compatible_assets.len() > 1 {
                let compatible_output = compatible_assets
                    .iter()
                    .map(|asset| format!("- {}", asset.name))
                    .collect::<Vec<_>>()
                    .join("\n");

                bail!(
                    "More than one compatible asset for {} v{} was found for your system. \
                    Aftman doesn't know how to handle this yet.\n\
                    Compatible assets:\n\
                    {}",
                    spec.name(),
                    release.version,
                    compatible_output
                );
            }

            log::info!("Downloading {} v{}...", spec.name(), release.version);
            let asset = &compatible_assets[0];
            let artifact = github.download_asset(&asset.url)?;

            self.install_artifact(&id, artifact).with_context(|| {
                format!(
                    "Could not install asset {} from tool {} release v{}",
                    asset.name,
                    id.name(),
                    release.version
                )
            })?;

            InstalledToolsCache::add(&installed_path, &id)
                .context("Could not write installed tools cache file")?;

            log::info!("{} v{} installed successfully.", id.name(), release.version);

            return Ok(id);
        }

        bail!("Could not find a compatible release for {spec}");
    }

    /// Ensure a tool with the given tool ID is installed.
    fn install_exact(&self, id: &ToolId) -> anyhow::Result<()> {
        let installed_path = self.storage_dir.join("installed.txt");
        let installed = InstalledToolsCache::read(&installed_path)?;
        let is_installed = installed.tools.contains(id);

        if is_installed {
            return Ok(());
        }

        log::info!("Installing tool: {id}");

        log::debug!("Fetching GitHub release...");
        let github = self.github.get_or_init(GitHubSource::new);
        let release = github.get_release(id)?;

        let compatible_assets = self.get_compatible_assets(&release);
        if compatible_assets.is_empty() {
            bail!("Tool {id} was found, but no assets were compatible with your system.");
        }

        if compatible_assets.len() > 1 {
            let compatible_output = compatible_assets
                .iter()
                .map(|asset| format!("- {}", asset.name))
                .collect::<Vec<_>>()
                .join("\n");

            bail!(
                "More than one compatible asset for {} v{} was found for your system. \
                Aftman doesn't know how to handle this yet.\n\
                Compatible assets:\n\
                {}",
                id.name(),
                release.version,
                compatible_output
            );
        }

        log::info!("Downloading {} v{}...", id.name(), release.version);
        let asset = &compatible_assets[0];
        let artifact = github.download_asset(&asset.url)?;

        self.install_artifact(id, artifact).with_context(|| {
            format!(
                "Could not install asset {} from tool {} release v{}",
                asset.name,
                id.name(),
                release.version
            )
        })?;

        InstalledToolsCache::add(&installed_path, id)
            .context("Could not write installed tools cache file")?;

        log::info!("{} v{} installed successfully.", id.name(), release.version);

        Ok(())
    }

    fn get_compatible_assets<'a>(&self, release: &'a Release) -> Vec<&'a Asset> {
        // If any assets list an OS or architecture that's compatible with
        // ours, we want to make that part of our filter criteria.
        let any_has_os = release
            .assets
            .iter()
            .any(|asset| asset.os.map(|os| os.compatible()).unwrap_or(false));
        let any_has_arch = release
            .assets
            .iter()
            .any(|asset| asset.arch.is_some() && asset.compatible());

        release
            .assets
            .iter()
            .filter(|asset| {
                // If any release has an OS that matched, filter out any
                // releases that don't match.
                let compatible_os = asset.os.map(|os| os.compatible()).unwrap_or(false);
                if any_has_os && !compatible_os {
                    return false;
                }

                // If any release has an OS and an architecture that matched
                // our platform, filter out any releases that don't match.
                let compatible = asset.compatible();
                if any_has_os && any_has_arch && !compatible {
                    return false;
                }

                true
            })
            .collect()
    }

    fn install_artifact(&self, id: &ToolId, artifact: impl Read + Seek) -> anyhow::Result<()> {
        let output_path = self.exe_path(id);
        let expected_name = format!("{}{EXE_SUFFIX}", id.name().name());

        let mut zip = zip::ZipArchive::new(artifact)?;
        for i in 0..zip.len() {
            let mut file = zip.by_index(i)?;
            if file.name() == expected_name {
                fs_err::create_dir_all(output_path.parent().unwrap())?;

                let mut output = BufWriter::new(File::create(output_path)?);
                io::copy(&mut file, &mut output)?;
                output.flush()?;

                return Ok(());
            }
        }

        bail!("executable {expected_name} not found in archive");
    }

    fn link(&self, alias: &ToolAlias) -> anyhow::Result<()> {
        let self_path =
            current_exe().context("Failed to discover path to the Aftman executable")?;

        let link_name = format!("{}{}", alias.as_ref(), EXE_SUFFIX);
        let link_path = self.bin_dir.join(link_name);

        fs_err::copy(self_path, link_path).context("Failed to create Aftman alias")?;
        Ok(())
    }

    fn exe_path(&self, id: &ToolId) -> PathBuf {
        let mut dir = self.storage_dir.clone();
        dir.push(id.name().scope());
        dir.push(id.name().name());
        dir.push(id.version().to_string());
        dir.push(format!("{}{}", id.name().name(), EXE_SUFFIX));
        dir
    }
}

#[derive(Debug)]
pub struct InstalledToolsCache {
    pub tools: BTreeSet<ToolId>,
}

impl InstalledToolsCache {
    pub fn read(path: &Path) -> anyhow::Result<Self> {
        let contents = match fs_err::read_to_string(path) {
            Ok(v) => v,
            Err(err) => {
                if err.kind() == io::ErrorKind::NotFound {
                    String::new()
                } else {
                    bail!(err);
                }
            }
        };

        let tools = contents
            .lines()
            .filter_map(|line| line.parse::<ToolId>().ok())
            .collect();

        Ok(Self { tools })
    }

    pub fn add(path: &Path, id: &ToolId) -> anyhow::Result<()> {
        let mut cache = Self::read(path)?;
        cache.tools.insert(id.clone());

        let mut output = String::new();
        for tool in cache.tools {
            writeln!(&mut output, "{}", tool).unwrap();
        }

        fs_err::write(path, output)?;
        Ok(())
    }
}
