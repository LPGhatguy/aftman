use std::collections::BTreeSet;
use std::fmt::Write;
use std::io;
use std::path::Path;

use anyhow::bail;

use crate::tool_name::ToolName;

#[derive(Debug)]
pub struct TrustCache {
    pub tools: BTreeSet<ToolName>,
}

impl TrustCache {
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
            .filter_map(|line| line.parse::<ToolName>().ok())
            .collect();

        Ok(Self { tools })
    }

    pub fn add(path: &Path, name: ToolName) -> anyhow::Result<()> {
        let mut cache = Self::read(path)?;
        cache.tools.insert(name);

        let mut output = String::new();
        for tool in cache.tools {
            writeln!(&mut output, "{}", tool).unwrap();
        }

        fs_err::write(path, output)?;
        Ok(())
    }
}
