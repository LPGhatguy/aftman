use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::format_err;

/// Defines the root that everything else in Aftman is stored relative to.
///
/// This type encourages good organization, helps us behave predictably, and
/// enables better tests for Aftman.
#[derive(Debug, Clone)]
pub struct Home {
    path: Arc<Path>,
}

impl Home {
    pub fn from_env() -> anyhow::Result<Self> {
        // Users can override the Aftman home directory via the AFTMAN_HOME
        // environment variable.
        if let Ok(var) = env::var("AFTMAN_HOME") {
            return Ok(Self::from_path(var));
        }

        let mut path =
            dirs::home_dir().ok_or_else(|| format_err!("Home directory could not be found."))?;

        path.push(".aftman");
        Ok(Self { path: path.into() })
    }

    #[allow(unused)]
    pub fn from_path<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            path: path.into().into(),
        }
    }

    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }
}
