use anyhow::{Context as _, Result};
use serde::Deserialize;

use crate::Dirs;

#[derive(Deserialize, Debug)]
pub(crate) struct Params {
    pub(crate) collections: String,
    pub(crate) count: u8,
    dest_dir: String,
}

impl Params {
    pub(crate) async fn read() -> Result<Self> {
        let path = format!("{}/params.toml", Dirs::configs_dir()?);
        let toml = tokio::fs::read_to_string(&path)
            .await
            .with_context(|| format!("failed to read params from {path}"))?;
        let params = toml::from_str(&toml).with_context(|| format!("invalid TOML in {path}"))?;
        Ok(params)
    }

    pub(crate) fn dest_dir(&self) -> Result<String> {
        Ok(format!("{}/{}", Dirs::home_dir()?, self.dest_dir))
    }
}
