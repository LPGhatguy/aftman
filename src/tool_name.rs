use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ToolName {
    inner: String,
}

impl AsRef<str> for ToolName {
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}
