mod cli;
mod config;
mod manifest;
mod tool_name;
mod tool_spec;

use std::env;

use anyhow::Context;
use structopt::StructOpt;

use crate::cli::Args;
use crate::config::initialize_global_config;
use crate::manifest::Manifest;

fn run() -> anyhow::Result<()> {
    let exe_name =
        env::current_exe().context("Failed to discover the name of the Aftman executable")?;

    let current_dir = env::current_dir().context("Failed to find current working directory")?;

    let manifests = Manifest::discover(&current_dir)?;
    println!("Manifests: {:#?}", manifests);

    // TODO: Resolve our current exe name against all manifests from our current
    // directory.

    initialize_global_config()?;

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
