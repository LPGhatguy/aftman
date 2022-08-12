mod auth;
mod cli;
mod config;
mod dirs;
mod home;
mod ident;
mod manifest;
mod process;
mod system_path;
mod tool_alias;
mod tool_id;
mod tool_name;
mod tool_source;
mod tool_spec;
mod tool_storage;
mod trust;

use std::env::{consts::EXE_SUFFIX, current_dir, current_exe};

use anyhow::{bail, format_err, Context};
use structopt::StructOpt;

use crate::auth::AuthManifest;
use crate::cli::Args;
use crate::home::Home;
use crate::manifest::Manifest;
use crate::tool_storage::ToolStorage;

fn run() -> anyhow::Result<()> {
    let home = Home::from_env()?;
    let tool_storage = ToolStorage::new(&home)?;
    let exe_name = current_exe_name()?;

    if exe_name != "aftman" {
        let start_dir = current_dir().context("Failed to find current working directory")?;
        let manifests = Manifest::discover(&home, &start_dir)?;

        for manifest in &manifests {
            if let Some(tool_id) = manifest.tools.get(exe_name.as_str()) {
                let args = std::env::args().skip(1).collect();
                tool_storage.run(tool_id, args)?;
                return Ok(());
            }
        }

        // If we're in Aftman's bin dir, we know for sure that we were supposed
        // to be an Aftman tool.
        let exe_path = current_exe()?;
        if exe_path.starts_with(&tool_storage.bin_dir) {
            let manifest_list = manifests
                .iter()
                .filter_map(|manifest| {
                    manifest
                        .path
                        .as_ref()
                        .map(|path| format!("- {}", path.display()))
                })
                .collect::<Vec<_>>()
                .join("\n");

            bail!("Tried to run an Aftman-managed version of {exe_name}, but no aftman.toml files list this tool.\n\
                To run {exe_name} from this directory, add it to one of these files:\n\
                {manifest_list}");
        }
    }

    Manifest::init_global(&home)?;
    AuthManifest::init(&home)?;
    system_path::init(&home)?;

    Args::from_args().run(&home, tool_storage)
}

fn current_exe_name() -> anyhow::Result<String> {
    let exe_path = current_exe().context("Failed to discover the name of the Aftman executable")?;
    let mut exe_name = exe_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format_err!("OS gave a funny result when asking for executable name"))?;

    exe_name = exe_name.strip_suffix(EXE_SUFFIX).unwrap_or(exe_name);

    Ok(exe_name.to_owned())
}

fn main() {
    let log_env = env_logger::Env::default().default_filter_or("info");

    env_logger::Builder::from_env(log_env)
        .format_module_path(false)
        .format_timestamp(None)
        .init();

    if let Err(err) = run() {
        eprintln!("Aftman error: {:?}", err);
        std::process::exit(1);
    }
}
