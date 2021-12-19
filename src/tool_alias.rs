use std::borrow::Borrow;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::ident::check_ident;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ToolAlias {
    name: String,
}

impl ToolAlias {
    pub fn new<S>(name: S) -> anyhow::Result<Self>
    where
        S: Into<String>,
    {
        let name = name.into();

        check_ident("Tool Name", &name)?;

        Ok(Self { name })
    }
}

impl FromStr for ToolAlias {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> anyhow::Result<Self> {
        ToolAlias::new(value)
    }
}

impl Borrow<str> for ToolAlias {
    fn borrow(&self) -> &str {
        &self.name.as_ref()
    }
}

impl AsRef<str> for ToolAlias {
    fn as_ref(&self) -> &str {
        self.name.as_ref()
    }
}
