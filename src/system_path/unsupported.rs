use std::path::Path;

use crate::home::Home;

pub fn init(_home: &Home) -> anyhow::Result<()> {
    Ok(())
}

pub fn add(_home: &Home) -> anyhow::Result<bool> {
    log::debug!("Not adding value to path because this platform is not supported.");
    Ok(false)
}
