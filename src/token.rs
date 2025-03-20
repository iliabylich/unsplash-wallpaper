use crate::Dirs;
use anyhow::{Context as _, Result};

pub(crate) struct Token;

impl Token {
    pub(crate) async fn read() -> Result<String> {
        let path = format!("{}/token", Dirs::configs_dir()?);
        let token = tokio::fs::read_to_string(&path)
            .await
            .with_context(|| format!("failed to read token from {path}"))?;
        Ok(token.trim().to_string())
    }
}
