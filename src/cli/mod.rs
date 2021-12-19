use structopt::StructOpt;

use crate::tool_alias::ToolAlias;
use crate::tool_spec::ToolSpec;
use crate::tool_storage::ToolStorage;

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

impl Args {
    pub fn run(self, tools: ToolStorage) -> anyhow::Result<()> {
        match self.subcommand {
            Subcommand::List(_) => todo!(),
            Subcommand::Add(sub) => sub.run(tools),
            Subcommand::Update(_) => todo!(),
        }
    }
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    List(ListSubcommand),
    Add(AddSubcommand),
    Update(UpdateSubcommand),
}

/// Lists all existing tools managed by Aftman.
#[derive(Debug, StructOpt)]
pub struct ListSubcommand {}

/// Adds a new tool to Aftman and install it.
#[derive(Debug, StructOpt)]
pub struct AddSubcommand {
    /// A tool spec describing where to get the tool and what version to
    /// install.
    pub tool_spec: ToolSpec,

    /// The name that will be used to run the tool.
    pub tool_alias: Option<ToolAlias>,
}

impl AddSubcommand {
    pub fn run(self, tools: ToolStorage) -> anyhow::Result<()> {
        tools.add(&self.tool_spec, self.tool_alias.as_ref())
    }
}

/// Updates one or more tools that are managed by Aftman.
///
/// Tools can be specified either by their alias or by their name.
///
/// If no tools are listed, Aftman will update all installed tools.
#[derive(Debug, StructOpt)]
pub struct UpdateSubcommand {
    /// One or more tools to update. If no tools are given, update all tools.
    pub aliases_or_specs: Vec<String>,

    /// Ignore semantic versioning and upgrade to the latest stable versions.
    #[structopt(long)]
    pub latest: bool,
}
