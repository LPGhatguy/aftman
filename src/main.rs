mod cli;
mod config;
mod ident;
mod manifest;
mod tool_alias;
mod tool_id;
mod tool_name;
mod tool_spec;
mod tool_storage;

use std::env;

use anyhow::Context;
use structopt::StructOpt;

use crate::cli::Args;
use crate::manifest::Manifest;
use crate::tool_storage::ToolStorage;

fn run() -> anyhow::Result<()> {
    let exe_name =
        env::current_exe().context("Failed to discover the name of the Aftman executable")?;

    let current_dir = env::current_dir().context("Failed to find current working directory")?;

    let manifests = Manifest::discover(&current_dir)?;
    println!("Manifests: {:#?}", manifests);

    // TODO: Resolve our current exe name against all manifests from our current
    // directory.

    Manifest::init_global()?;
    ToolStorage::init()?;

    let args = Args::from_args();

    println!("Args: {:#?}", args);

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{:?}", err);
        std::process::exit(1);
    }
}
