use std::io::Read;

use anyhow::Context;
use reqwest::{
    blocking::Client,
    header::{ACCEPT, USER_AGENT},
};
use serde::{Deserialize, Serialize};

use crate::tool_spec::ToolSpec;

pub fn download_tool(spec: &ToolSpec) -> anyhow::Result<impl Read> {
    let releases = get_releases(spec.name().as_ref())?;

    let dummy: &[u8] = &[];
    Ok(dummy)
}

pub fn get_releases(repo: &str) -> anyhow::Result<Vec<Release>> {
    let client = Client::new();

    let url = format!("https://api.github.com/repos/{}/releases", repo);
    let builder = client.get(&url).header(USER_AGENT, "LPGhatguy/aftman");

    // TODO: Authorization

    let response_body = builder.send()?.text()?;

    let releases: Vec<Release> = serde_json::from_str(&response_body)
        .with_context(|| format!("Unexpected GitHub API response: {}", response_body))?;

    Ok(releases)
}

fn download_asset(url: &str) -> anyhow::Result<impl Read> {
    let client = Client::new();

    let builder = client
        .get(url)
        .header(USER_AGENT, "LPGhatguy/aftman")
        .header(ACCEPT, "application/octet-stream");

    // TODO: Authorization

    Ok(builder.send()?)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub prerelease: bool,
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseAsset {
    pub url: String,
    pub name: String,
}
