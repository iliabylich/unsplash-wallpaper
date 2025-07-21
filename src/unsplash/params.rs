use crate::Dirs;
use anyhow::{Context as _, Result};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub(crate) struct Params {
    pub(crate) collections: String,
    pub(crate) count: u8,
    dest_dir: String,
}

impl Params {
    pub(crate) async fn read() -> Result<Self> {
        let path = Dirs::config_dir().join("params.toml");
        let toml = tokio::fs::read_to_string(&path)
            .await
            .with_context(|| format!("failed to read params from {path:?}"))?;
        let params = toml::from_str(&toml).with_context(|| format!("invalid TOML in {path:?}"))?;
        Ok(params)
    }

    pub(crate) fn full_dest_dir(&self) -> PathBuf {
        Dirs::home().join(&self.dest_dir)
    }
}
