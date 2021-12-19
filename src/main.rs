mod cli;
mod config;
mod ident;
mod manifest;
mod tool_alias;
mod tool_id;
mod tool_name;
mod tool_source;
mod tool_spec;
mod tool_storage;

use std::env::{consts::EXE_SUFFIX, current_dir, current_exe};

use anyhow::{format_err, Context};
use structopt::StructOpt;

use crate::cli::Args;
use crate::manifest::Manifest;
use crate::tool_storage::ToolStorage;

fn run() -> anyhow::Result<()> {
    let exe_name = current_exe_name()?;
    let start_dir = current_dir().context("Failed to find current working directory")?;
    let manifests = Manifest::discover(&start_dir)?;

    let tool_storage = ToolStorage::init()?;

    for manifest in &manifests {
        if let Some(tool_id) = manifest.tools.get(exe_name.as_str()) {
            let args = std::env::args().collect();
            tool_storage.run(tool_id, args)?;
            return Ok(());
        }
    }

    Manifest::init_global()?;

    Args::from_args().run(tool_storage)
}

fn current_exe_name() -> anyhow::Result<String> {
    let exe_path = current_exe().context("Failed to discover the name of the Aftman executable")?;
    let mut exe_name = exe_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format_err!("OS gave a funny result when asking for executable name"))?;

    if exe_name.ends_with(EXE_SUFFIX) {
        exe_name = &exe_name[..exe_name.len() - EXE_SUFFIX.len()];
    }

    Ok(exe_name.to_owned())
}

fn main() {
    env_logger::init();

    if let Err(err) = run() {
        eprintln!("Aftman error: {:?}", err);
        std::process::exit(1);
    }
}
