mod cli;
mod manifest;
mod tool_name;
mod tool_spec;

use structopt::StructOpt;

use crate::cli::Args;

fn main() {
    let args = Args::from_args();

    println!("{:#?}", args);
}
